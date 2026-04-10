#![allow(unused, unused_imports)]




use std::{net::{SocketAddr, TcpStream}, sync::Arc, fmt::Debug};
use anyhow::{anyhow, bail};
use axum::{extract::{WebSocketUpgrade, ConnectInfo, Query, ws::Message}, TypedHeader, response::IntoResponse, body::{HttpBody, self}};
use bonsaidb::core::schema::SerializedCollection;
use futures::{StreamExt, SinkExt, stream::SplitStream};
use serde_json::from_str;
use tokio::sync::{mpsc::Sender, Mutex};
use tungstenite::handshake::server::create_response;
use super::*;
use chattyApi::{channelHander::{self, ChannelHandler, IncomingMessage}, server::*, database::Account, TargetIdAndTokenStr, DisplayIfError};


struct IdToken{
    pub id_token: TargetIdAndToken,
    pub addr: SocketAddr,

}
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    // id_token: Option<Query<String>>,
    request : Request<body::Body>
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    println!("`{user_agent}` at {addr} connected.");
    let uri = request.uri();
    let query: Query<TargetIdAndTokenStr> = Query::try_from_uri(uri).unwrap();
    // dbg!(&request);
    // dbg!(&query);
    let id_token = TargetIdAndToken{
        id: query.id.parse::<u128>().unwrap(),
        token : query.token.parse::<u128>().unwrap()
    };
    let info = IdToken{addr, id_token};
    ws.on_upgrade(move |socket| {
        async{
            sync_user(socket ,info).await;
        }
    })
    // create_response(request).unwrap()
    // println!("all DONE MF"); 

}
// pub async fn sync_user_f(socket: WebSocket, json: Json<ConnectionJson>){
//     spawn( async move{
//         sync_user(socket, json).await;
//     }
//     );
// }

async fn sync_user(socket: WebSocket, info : IdToken)-> R<()>{
    {
        let db = chattyApi::get_db().await.unwrap();
        let account = Account::get_async(&info.id_token.id, &db).await.unwrap().unwrap().contents;
        if info.id_token.token != account.auth_token{
            dbg!(&account);
            println!("with client token -> {}", info.id_token.token);
            panic!("token missmatch");
        }
    }
    let sender = STATIC.SENDER.clone();
    let (mut ws_sende,mut ws_receiver ) = socket.split();
    
    let ws_sender = Arc::new(Mutex::new(ws_sende));
    let id = info.id_token.id;
    let token = info.id_token.token;
    let addr= info.addr;
    // let sender2= sender.clone();
    let mut channel_handler = ChannelHandler{
      ws_sender : ws_sender.clone(),
        sender: sender.clone(),
        from : addr.clone(),
        from_id : id.clone(),
        token : token.clone(),
    };
    channel_handler.send_ws_text("syncing user to db..".to_string()).await?;
    channel_handler.send_start_connection().await?;
    channel_handler.send_ws_text("user synced".to_string()).await?;
    let mut channel_handler2 = channel_handler.clone();
    let thread = spawn(async move {
        println!("help mommy");
        let res=  start_sock(channel_handler, ws_receiver).await;
        println!("SOMETHING HAPPENED {:?}",res);
        channel_handler2.send_close().await;
    });
    use tokio::time::Duration;
    tokio::time::sleep(Duration::from_secs(20000)).await;
    // loop{

    // }

    Ok(())

}
// pub async fn sync_user2(socket: WebSocket, json : Json<ConnectionJson>)-> R<()>{

// }
async fn start_sock(mut channel_handler:  ChannelHandler, mut ws_receiver: SplitStream<WebSocket>)-> R<()>{
        channel_handler.send_ws_text("in da loopidy doo".to_string()).await?;        
        loop{
            println!("looping in start sock..");
            let message = ws_receiver.next().await;
            if message.is_none(){
                break
            }
            // println!("recieved message, {:?}", message);
            let message = message.unwrap();
            if message.is_err(){
                continue
            }
            let mut message = message.unwrap();
            channel_handler.send_ws_text("got a message".to_string()).await.display_if_err();        
            // dbg!(message.to_text());
            if let  Ok(text) = message.to_text(){

                match from_str::<IncomingMessage>(text){
                   Ok(incomming_message)=>{ 
                    // println!("accepted message {:?}", &incomming_message);
                    let r = channel_handler.send_from_incoming_message(incomming_message).await.display_if_err();
                    // r.displayIfError();
                    channel_handler.send_ws_text("got a message from ya :D".to_string()).await.display_if_err();
                    continue        
                    }
                   Err(err)=> {dbg!(err);}
                }
            }
            // let m = ChannelMessage{
            //     message_type: MessageType::ErrorMessage,
            //     from_id :id,
            //     from: addr,
            //     to_chat_room : 0,
            //     to_room :0
            // }
            // sender.send()
            // ws_sender.blocking_lock().send(Message::Text("invalid inputs!".to_string())).await;

            // channel_handler.send_ws_text("breaking".to_string()).await?;        
            // break

        };
        Ok(())


}
