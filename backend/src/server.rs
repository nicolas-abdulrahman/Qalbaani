use std::{collections::HashMap, sync::{Arc}, time::{SystemTime, Duration}, io::{BufReader, Cursor}, fmt::format, pin::Pin, future::ready};

use image::{io::Reader as ImageReader, codecs::png::PngDecoder, ImageFormat};
use anyhow::{anyhow, bail};
use axum::{extract::ws::{WebSocket, Message}, Json};
use bonsaidb::core::schema::SerializedCollection;
use futures::{stream::SplitSink, SinkExt, Future};
use image::ImageDecoder;
use tokio::{sync::{mpsc::Receiver, Mutex, MutexGuard}, spawn, fs, time::Sleep};

use crate::{channelHander::{ChannelMessage, MessageType, ImageMessage}, R, database::{Account, Room, AssociatedRoomKey, Profile, AssociatedMessageKey, ChatRoom}, get_db, log, add_friend, TargetIdAndToken, DisplayIfError, AddFriend};

pub struct  Server{
    pub  users_connected : Mutex<HashMap<u128, Arc<Mutex<UserConnection>>>>,
    pub rooms: Mutex<HashMap<u64, Arc<Mutex<ChatRoomConnection>>>>,
    pub receiver : Arc<Mutex<Receiver<ChannelMessage>>>,
}

#[derive(Debug)]
pub struct UserConnection{
    pub connections : Vec<Connection>,
    // pub account: Account,
    pub id: u128,
    pub username: String,
    pub profile: Profile,
    pub profile_id : u64,

}
#[derive(Debug)]
pub struct  Connection{
    pub ws_sender :  Arc<Mutex<SplitSink<WebSocket, Message>>>,

}

#[derive(Debug)]
pub struct ChatRoomConnection{
    pub user_connections : Vec<Arc<Mutex<UserConnection>>>,
    pub rooms : Vec<RoomConnection>,
    pub chat_room: ChatRoom,
}

#[derive(Debug)]
pub struct RoomConnection{
    pub key : AssociatedRoomKey,
    pub room: Room,
    // pub messages: u32,
    pub buffer: Vec<crate::database::Message>
}
impl  UserConnection {

    // pub  async fn new(server: &Server, id: u128 )-> R<Self>{

    // }
    
}

// pub trait TryFor<T>{
//     // fn try_lock_for<'a>(&self, miliseconds: u32)-> Pin<Box<dyn Future<Output = R<MutexGuard<'a, T>>>>>;
//
//     fn lock_notify<'a: 'static>(&'a self) -> Pin<Box<dyn Future<Output = R<MutexGuard<'a, T>>>>> ;
// }
//
// impl<T> TryFor<T> for Mutex<T>{
//     // fn try_lock_for<'a>(&self, miliseconds: u32)-> Pin<Box<dyn Future<Output = R<MutexGuard<'a, T>>>>> {
//         // let n = self.try_lock();
//         // if n.is_ok(){
//         //     return Box::pin(ready(Ok(n.unwrap())))
//         // }
//         // return Box::pin(ready(Err(anyhow!("errror"))))
//         // bail!("couldnt aquire lock");
//       
//     // }
//     fn lock_notify<'a: 'static>(&'a self) -> Pin<Box<dyn Future<Output = R<MutexGuard<'a, T>>>>> {
//         let n = self.try_lock();
//         if n.is_ok(){
//             return Box::pin(ready(Ok(n.unwrap())))
//         }
//         async{
//              tokio::time::sleep(Duration::from_millis(10)).await;
//         };
//         return Box::pin(ready(Err(anyhow!("errror"))))
//       
//     }
//
// }
impl<'ser> Server{

    pub async fn handle_message(&self, message: ChannelMessage)->R<()>{
        match message.message_type{
            MessageType::add_connection =>{
                println!("SERVER IN ADD CONNECTION");
                let r = self.add_connection(message).await;
                if r.is_err(){
                    log(format!("{:?}", r.unwrap_err())).display_if_err();
                }

            }
            MessageType::add_friend => {
                println!("adding friend..");
                let ws =  message.ws_sender.ok_or(anyhow!("pass me the ws_handler sussy_baka"))?.clone();
                let id_token = TargetIdAndToken{id: message.from_id, token: message.token};
                let r = add_friend(id_token, message.data.clone()).await?;
                println!("added friend {:?}", &r);
                let user_connection_hashmap = self.users_connected.lock().await;
                // dbg!(&user_connection_hashmap);
                let user_connection = user_connection_hashmap.get(&message.from_id).ok_or(anyhow!(
                    "user expected to be connected"))?.clone();
                let mut vec = vec![user_connection.clone()];
                println!("here..");
                if let Some(friend_connection) = user_connection_hashmap.get(&r.friend_id){
                    vec.push(friend_connection.clone());
                }
                drop(user_connection_hashmap);
                let chat_room_connection = ChatRoomConnection::new(r.chat_room, vec).await?;
                self.rooms.lock().await.insert(r.chat_room, Arc::new(Mutex::new(chat_room_connection)));

                println!("now_here..");
                
                // if r.is_err(){
                    // ws_handler.send(Message::Text(format!("error adding friend {:?}", r.unwrap_err()))).await?;
                    // Message::Text(format!("error adding friend {:?}", r.unwrap_err())).await;
                // }
                // else{
                let mut ws_handler = ws.lock().await;
                println!("sending message..");
                // let json = Json(r);
                let json =serde_json::to_string::<AddFriend>(&r)?;
                ws_handler.send(Message::Text(json)).await?;
                // }
            }
            MessageType::get_messages =>{
                println!("SERVER GET MESSAGES");
                let chat_room_id = message.to_chat_room.ok_or(anyhow!("couldnt find chat room"))?;
                let room_id = message.to_room.unwrap_or(0);
                let db = get_db().await?;
                let key = AssociatedRoomKey{
                    room_id,
                    chat_room_id,
                };
                let room = Room::get_async(&key, &db).await?.ok_or(anyhow!("couldnt find room again"))?;
                let mes_key = AssociatedMessageKey{
                    room_id,
                    chat_room_id,
                    message_id: 0,
                };
                let ws_sender = message.ws_sender.ok_or(anyhow!("gimme ws sender baka"))?.clone();
                // println!("message is DAN DAN DAN DAAN {:?}",&message);
                let chat_room_hashmap = self.rooms.lock().await;
                let chat_r = chat_room_hashmap.get(&chat_room_id).ok_or(anyhow!("error retrieving chat room connection"))?.clone();
                drop(chat_room_hashmap);
                let mut string = String::new();
                {
                    let mut chat_room_connection = chat_r.lock().await;
                    let room_connection = chat_room_connection.get_room(room_id).ok_or(anyhow!("failed in get_room, please what am i doing?"))?;
                    // let messages = format!("{:?}",room_connection.buffer);
                    string.push('[');
                    for message in room_connection.buffer.iter(){
                        string.push_str(serde_json::to_string(message)?.as_str());
                        string.push(',');
                    }
                    string.push(']');
                }
                // let message = crate::database::Message::get_async(&mes_key, &db).await?.ok_or(anyhow!("GET MESSAGE err, couldnt get message"))?;
                ws_sender.lock().await.send(Message::Text(string)).await?;
            }
            MessageType::send_message => {
                log("\nSERVER got a send message\n".to_string()).display_if_err();
                let r = self.send_message(message).await;
                if r.is_err(){
                    log(format!("error in sending message {:?}", r.unwrap_err())).display_if_err();
                }
            }
            MessageType::upload_image =>{
                log("\nSERVER got a upload image\n".to_string()).display_if_err();
                let image_message = serde_json::from_str::<ImageMessage>(&message.data)?;
                let ws_sender= message.ws_sender.ok_or(anyhow!("didint recieve ws_swnder"))?;
                // dbg!(&image_message);
                // BufReader::new(image_message.bytes);
                let first_8_bytes = image_message.bytes.get(0..8).unwrap();
                log(format!(
                        "\nfirst 8 bytes:{:?}\n", first_8_bytes)).display_if_err();
                let mut img = ImageReader::new(Cursor::new(image_message.bytes));
                img.set_format(ImageFormat::Png);
                let image = img.decode()?;
                log("\ndecoded image sucessufuly\n".to_string()).display_if_err();
                // let file = fil
                // img.write_to(w, format)
                let uid = uuid::Uuid::new_v4().to_string();
                fs::create_dir("assets/".to_string() + &uid.to_string()).await?;
                let path = format!("assets/{}/{}", uid, image_message.name);
                image.save(&path)?;
                // self.notify_users_at_room
                
                ws_sender.lock().await.send(Message::Text(path)).await?;

            }
            MessageType::close_socket => {
                bail!("Not implemented")
            }
        }
        Ok(())
    }

    pub async fn start_server(self: Arc<Self>){
        loop{
            log("server loopity doo\n".to_string()).display_if_err();
            let reciv = self.receiver.clone();
            let mut rec_lock = reciv.lock().await;
            log("received lock\n".to_string()).display_if_err();
            let rec = rec_lock.recv().await;
            log("server Got response".to_string()).display_if_err();
            if rec.is_none(){
                println!("no message in receiver");
                continue
            }
            // drop(reciv);
            let message = rec.unwrap();
            let server_ref  = self.clone(); 
            spawn(async move{
                server_ref.handle_message(message).await.display_if_err();
            });
            // println!("SERVER: receiver a message{:?}", message);
        }
    }
    pub async fn assert_has_permission_to_access(from_id: u128, chat_room: Arc<Mutex<ChatRoomConnection>>)-> R<()>{
        let chat_room_connection = chat_room.lock().await;
        for id in chat_room_connection.chat_room.users.iter(){
            if id == &from_id{
                return Ok(())
            }
        }
        bail!("user is not listed in chat_room!! you sussy baka")
    }
    
    pub async fn send_message(&self, message: ChannelMessage) -> R<()>{
        
        // let room_lock = self.rooms.lock().await;
        // drop()
        // let chat_room = room_lock.get(&chat_room_id).ok_or(anyhow!("uknown room?"))?;
        let chat_room_id = message.to_chat_room.ok_or(anyhow!("couldnt unwrap to_chat_room as ok"))?;
        let room_id : u32 = message.to_room.unwrap_or(0);
        let chat_room_hashmap = self.rooms.lock().await;
        let chat_r = chat_room_hashmap.get(&chat_room_id).ok_or(anyhow!("aa error retrieving chat room connection"))?.clone();
        drop(chat_room_hashmap);
        Server::assert_has_permission_to_access(message.from_id, chat_r.clone() ).await?;
        let db = get_db().await?;
        let key = AssociatedRoomKey{
            room_id,
            chat_room_id: chat_room_id.clone()
        };
        let user_connection_hashmap = self.users_connected.lock().await;
        let user_c =  user_connection_hashmap.get(&message.from_id).ok_or(anyhow!("expected user connection here"))?.clone();
        drop(user_connection_hashmap);
        let room = Room::get_async(&key, &db).await?.ok_or(anyhow!("couldnt find key in database"))?;
        let id = message.from_id;
        let timestamp = SystemTime::now();

        println!("the time is {:?}", &timestamp);
        let message_id = room.contents.messages;
        let mes_key = AssociatedMessageKey{
            room_id,
            chat_room_id: chat_room_id.clone(),
            message_id,
        };
        let user_connection = user_c.lock().await;
        let mes = crate::database::Message{
            id:mes_key,
            timestamp,
            profile: user_connection.profile_id,
            contents: message.data.clone(),
        };
        drop(user_connection);
        let mut chat_room_connection = chat_r.lock().await;
        let  room_connection = chat_room_connection.get_room(room_id).ok_or(anyhow!("aa failed in get_room, please what am i doing?"))?;
        room_connection.buffer.push(mes);

        
        // let room_connection = self.get_room(chat_room_id, room_id);
        // Message::from()
        // drop(user_connection_hashmap);
        let data = message.data;
        // dbg!(&chat_room);
        for v in chat_room_connection.user_connections.iter(){
            // println!("looping through!!");
            let user_connection = v.lock().await;

            // println!("got the lock {:?}", &user_connection);
            // if user_connection.account.id == id{
                // return 
            // }
            for con in user_connection.connections.iter(){
                    let r = con.ws_sender.lock().await.send(Message::Text(data.clone())).await;
                    if r.is_err(){
                        log(format!("error in the map {:?}", r)).display_if_err();
                }
            }
        }
        Ok(())
    }

    pub async fn add_connection(&self, message: ChannelMessage)-> R<()>{
        let mut user_connection_hashmap = self.users_connected.lock().await;
        let user_connection = user_connection_hashmap.get(&message.from_id);
        if user_connection.is_none(){
            // drop(user_connection_hashmap);
            let ws_sender= message.ws_sender.ok_or(anyhow!("gimme the ws_sender"))?;
            let db = get_db().await?;
            let id = message.from_id;
            let account = Account::get_async(&id, &db).await?.ok_or(anyhow!("couldnt get account"))?.contents;
            let profile = Profile::get_async(&account.main_profile, &db).await?.ok_or(anyhow!("couldnt get profile"))?.contents;
            let user_con = UserConnection { 
                connections: vec!(Connection{
                    ws_sender: ws_sender.clone()
                }),
                id: account.id,
                username: account.username,
                profile,
                profile_id: account.main_profile,
            };
            let user_connection = Arc::new(Mutex::new(user_con));
            // let mut user_connection_hashmap = self.users_connected.lock().await;
            user_connection_hashmap.insert(id, user_connection.clone());
            println!("user added to the hashmap :D");
            let user_c = user_connection_hashmap.get(&id).ok_or(anyhow!("wtf going on?? im confused"))?.clone();
            drop(user_connection_hashmap);
            let mut chat_room_hashmap = self.rooms.lock().await;
            println!("GOT THE HASHMAP LOCK");
            for chat_room in account.chat_rooms.iter(){
                let chat_room_connection = chat_room_hashmap.get_mut(chat_room);
                println!("GOT THE HASHMAP LOCK,NOW LOOPING");
                if chat_room_connection.is_none(){
                    let chat_room_connection = ChatRoomConnection::new(chat_room.clone(), vec![user_c.clone()]).await?;
                    chat_room_hashmap.insert(chat_room.clone(), Arc::new(Mutex::new(chat_room_connection)));
                    println!("GOT THE HASHMAP LOCK, INSERTED INTO HASHMAP");
                    continue
                }
                let mut room = chat_room_connection.unwrap().lock().await;
                room.add_connection(user_c.clone())?;
            }
            println!("finished adding new connection..");
            // let new_user = Arc::new(Mutex::new(UserConnection::new(self,message.from_id).await?));
            // let mut user_l = self.users_connected.lock().await;
            // println!("adding user right ABOUT now..");
            // user_l.insert(message.from_id, new_user.clone());
            // println!("added user into hashmap ");
           return Ok(())

        }
        let user_c = user_connection_hashmap.get(&message.from_id).ok_or(anyhow!("wtf going on??"))?;
        let mut user_connection =  user_c.lock().await;
        let ws_sender = message.ws_sender.ok_or(anyhow!("that shouldnt fail"))?;
    
        user_connection.connections.push(Connection{ws_sender});
        Ok(())

    }
}

impl ChatRoomConnection{
    pub async fn new(chat_room_id: u64, user_connections : Vec<Arc<Mutex<UserConnection>>>)-> R<Self>{
        let db = get_db().await?;
        let chat_room= ChatRoom::get_async(&chat_room_id, &db).await?.ok_or(anyhow!("invalid chat room"))?.contents;
        let key = AssociatedRoomKey{
            chat_room_id,
            room_id: 0,
        };
        let room = Room::get_async(&key,&db).await?.ok_or(anyhow!("failed to get room"))?;
        let room_connection = RoomConnection::new(key, room.contents).await;
        // let room = room::new(0);
        Ok(ChatRoomConnection{
            user_connections,
            rooms: vec![room_connection],
            chat_room,
        })
    }
    pub fn add_connection(&mut self, user_connection: Arc<Mutex<UserConnection>>)-> R<()>{
        self.user_connections.push(user_connection);
        Ok(())
    }
    pub fn get_room(&mut self, room_id: u32)-> Option<&mut RoomConnection>{
        for room in self.rooms.iter_mut(){
            if room.key.room_id == room_id{
                return Some(room)
            }
        }
        None
    }
}

impl RoomConnection{
    pub async fn new(key: AssociatedRoomKey,room: Room)-> Self{
        RoomConnection{
            key,
            room,
            buffer: vec![],
        }

    }

    pub async fn dump_to_db(&mut self)-> R<()>{
        let db = get_db().await?;
        let mut chat_room = Room::get_async(&self.key, &db).await?.ok_or(anyhow!("unexpected error, couldnt find room"))?;
        // chat_room. 
        let mut messages = chat_room.contents.messages;
        while self.buffer.len()>0{
            let message = self.buffer.swap_remove(0);
            if  message.push_into_async(&db).await.is_ok(){
                chat_room.contents.messages +=1;
            }
        }
        // for message in self.buffer.iter(){
            // if  message.push_into_async(&db).await.is_ok(){
                // chat_room.contents.messages +=1;
            // }

        // }
        chat_room.contents.messages = messages;
        chat_room.update_async(&db).await?;
        Ok(())

    }
}

// trait InsertIfEmpty<T> {
//     fn insert_if_empty(&self, key: T)-> R<()> ;
// }
// impl<K, T> InsertIfEmpty<T> for HashMap<K,  Arc<Mutex<UserConnection>>>
// where 
//     K: Into<u128>, 
//    
// {
//     fn insert_if_empty(&mut self, T)-> R<()> {
//         self.insert()
//     }
//
// }
