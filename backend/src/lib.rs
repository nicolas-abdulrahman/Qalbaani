pub mod database;
pub mod server;
pub mod channelHander;

use axum::routing::any;
use serde::{Deserialize, Serialize};
use anyhow::{ensure, bail, anyhow};
use bonsaidb::core::connection::{StorageConnection, AsyncStorageConnection};
use bonsaidb::core::document::{CollectionDocument, self, Document, HasHeader};
use bonsaidb::core::schema::view::map::{MappedDocument, MappedDocuments};
use bonsaidb::core::schema::{SerializedView, SerializedCollection};
use bonsaidb::local::config::{StorageConfiguration, Builder};
use database::*;
use bonsaidb::core;

use bonsaidb::local::{AsyncStorage, AsyncDatabase};
use regex::Regex;
use serde_json::json;
use std::clone;
use std::fmt::Debug;
use std::fs::OpenOptions;
use std::io::{ErrorKind, Error, Write};
// use channelHander::*;


#[derive(Debug)]
enum MyError {
    SpeicialChars,
    AlreadyExists,
    Custom(String),
}
// #[derive(Serialize, Deserialize)]
// pub struct Token{
//
// }
impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::SpeicialChars => write!(f, "Speical characthers found"),
            MyError::AlreadyExists => write!(f, "Data already exists on database"),
            MyError::Custom(msg) => write!(f, "Custom error: {}", msg),
        }
    }
}
impl std::error::Error for MyError {}

pub type R<T> = Result<T, anyhow::Error>;

pub async fn create_database() -> Result<(), anyhow::Error>
{
    std::fs::remove_dir_all("my-db.bonsaidb")?; 
    
    let storage = AsyncStorage::open(StorageConfiguration::new("my-db.bonsaidb").with_schema::<MainSchema>()?).await?;
    
    //let storage = AsyncStorage::create_database(&self, name, only_if_needed)
    storage.create_database::<MainSchema>("my-db", true).await?;
    let db = storage.database::<MainSchema>("my-db").await?;
    create_default_profile(String::from("127.0.0.1:3002/assets/default_profile.png"));


    Ok(())
    
}


fn validate_username(username:  &mut String, pattern: Regex)->Result<(), anyhow::Error>{
    // let mut new_username = username.replace(" ", "");
    username.retain(|c| !c.is_whitespace());
    if pattern.is_match(username){
        println!("balinig");
        anyhow::bail!("has special chars");
    }
    else {
        Ok(())
    }
    // Ok(())


}

#[derive(Debug, clone::Clone, Copy, Serialize, Deserialize )]
pub struct TargetIdAndToken{
    pub id: u128,
    pub token: u128,
}
#[derive(Debug, clone::Clone, Serialize, Deserialize )]
pub struct TargetIdAndTokenStr{
    pub id: String,
    pub token: String,
}
pub async fn register_user(mut username:  String, password: String) -> Result<TargetIdAndToken, anyhow::Error> {
    anyhow::ensure!(password.len()> 6);
    let storage = AsyncStorage::open(StorageConfiguration::new("my-db.bonsaidb").with_schema::<MainSchema>()?).await?;
    let db = storage.database::<MainSchema>("my-db").await?;
    let pattern = Regex::new(r"[^a-zA-Z0-9~]").unwrap();
    validate_username(&mut username, pattern)?;
    


    let user_id = uuid::Uuid::new_v4().as_u128();

    let main_profile = Profile{

        name: username.clone(),
        image: "default_profile.png".to_string(),
        name_color: "#ffffff".to_string(),
        bubble_color: "#ffffff".to_string(),
        status: "".to_string(),
        owner: user_id,
    }.push_into_async(&db).await?;
    
    let user = Account{
        id : user_id,
        username,
        password,
        friends: vec!(),
        chat_rooms: vec!(),
        auth_token: uuid::Uuid::new_v4().as_u128(),
        main_profile : main_profile.header.id,
        profiles : vec!(main_profile.header.id),
        friend_requests : vec!(),
    };
    let has_match = SortAccounts::entries_async(&db).with_key(&user.username.to_lowercase()).query_with_collection_docs().await?;
    // println!("matches : {}", has_match);
    if  let Some(account) =has_match.into_iter().next() {
        println!("found a account with the same username, {}", account.document.contents.username);
        return Err(Error::new(ErrorKind::InvalidData, "account with the same name".to_string()).into())
    }
    else {
        let id = user.id.clone();
        let auth_token = user.auth_token.clone();
        // println!("inserting a new user {} into db", &user.username);
        user.push_into_async(&db).await?;
        // let (session_id, token) = server.take_offline_session()?;
        // let token = uuid::Uuid::new_v4().as_u128();
        Ok(TargetIdAndToken {id, token: auth_token })

    }

}



pub enum MyParam{
    U128value(u128),
    StringValue(String),
}


pub async fn create_default_profile(img_path: String)-> R<()>{
    let db = get_db().await?;
    Profile{
        name: String::from("???"),
        image : img_path,
        name_color : "#00ffff".to_string(),
        bubble_color : "#ffffff".to_string(),
        status : "".to_string(),
        owner : 0
    }.push_into_async(&db).await?;
    Ok(())

}
pub async fn get_db()-> Result<AsyncDatabase, anyhow::Error>{
    let storage = AsyncStorage::open(StorageConfiguration::new("my-db.bonsaidb").with_schema::<MainSchema>()?).await?;
    let db = storage.database::<MainSchema>("my-db").await?;
    Ok(db)
}

fn validate_token(user: TargetIdAndToken)-> R<()>{
    Ok(())
}

// impl From<String> for u128{
// }
#[derive(Debug, Deserialize, Serialize)]
pub struct AddFriend{
    pub friend_id: u128,
    pub chat_room: u64,

}
pub async fn add_friend(user : TargetIdAndToken, friend: String)-> Result<AddFriend, anyhow::Error>
     {
    validate_token(user)?;
    let db = get_db().await?;
      
    async fn add_friend_by_doc(user : TargetIdAndToken, mut friend_doc : CollectionDocument<Account>, db: AsyncDatabase)-> R<AddFriend>{
        let mut user_doc = Account::get_async(&user.id, &db).await?.ok_or(anyhow!("invalid user id"))?;

        user_doc.contents.friends.push(friend_doc.header.id);
        friend_doc.contents.friends.push(user_doc.header.id);
        let private_chat = ChatRoom{
            name : "new chat ^w^".to_string(),
            description : "i know your ip owo".to_string(),
            chat_type : ChatType::Private,
            rooms : vec![0],
            users : vec![friend_doc.header.id, user_doc.header.id],
        }.push_into_async(&db).await?;
        let chat_room_id = private_chat.header.id;

        let default_room = Room{
            id : AssociatedRoomKey { room_id: 0 , chat_room_id: private_chat.header.id },
            name: "our den".to_string(),
            description: "we chill here".to_string(),
            background : "#ffffff".to_string(),
            messages : 0,
        }.push_into_async(&db).await?;
        // let id = default_room.header.id.room_id
        user_doc.contents.chat_rooms.push(private_chat.header.id);
        friend_doc.contents.chat_rooms.push(private_chat.header.id);
        user_doc.update_async(&db).await?;
        friend_doc.update_async(&db).await?;
        log(format!("added friends from {}, to {}, 
                    current user doc {:?}", user_doc.contents.username, friend_doc.contents.username, user_doc.contents)).display_if_err();
        // let json = json!({
            // "chat_room":chat_room_id
        // });
        let add_f = AddFriend{
            chat_room: chat_room_id,
            friend_id: friend_doc.header.id,
        };
        return Ok(add_f)

   }

    
    // let clone = friend_id.clone();
    let val = friend.parse::<u128>();
    if val.is_ok(){
            let u:u128 = val.unwrap();
            let mut doc = Account::get_async(&u, &db).await?.ok_or(anyhow!("couldnt find friend"))?;
            let json = add_friend_by_doc(user, doc, db).await?;
            Ok(json)
    }
    else{
        let u:String = friend;

        let friend_id =  SortAccounts::entries_async(&db).with_key(&u.to_lowercase()).query().await?
            .first().ok_or(anyhow!("tried to add a friend that didnt exist"))?.value.clone();
        // let friend_id = friend.value.clone();
        let doc = Account::get_async(&friend_id, &db).await?.ok_or(anyhow!("unexpected error"))?;
        // let doc= friend.into_iter().next().ok_or(anyhow!("couldnt find friend"))?.document;
        let json = add_friend_by_doc(user, doc, db).await?;
        Ok(json)
    }

    

             
}


pub trait DisplayIfError{
    fn display_if_err(&mut self);
}

impl<T,F> DisplayIfError for Result<T, F>
where F: Debug, T: Debug{
    fn display_if_err(&mut self) {
        if self.is_err(){
            println!("ERROR {:?}", self.as_ref().unwrap_err());
        }
    }
}


fn log(contents: String)-> R<()>{
    let mut file = OpenOptions::new()
        .append(true)
        .open("log")?;
    file.write_all(contents.as_bytes());
    // let r = std::fs::("log", contents);
    // if r.is_err(){
        // println!("failed to write file to log");
    // }
    Ok(())

}


async fn create_token_for(id: u128)-> Result<u128, anyhow::Error>{
    Ok(uuid::Uuid::new_v4().as_u128())
}

pub async fn login(username: String, password: String)-> Result<TargetIdAndToken, anyhow::Error>{
    let db = get_db().await?;
    let entry =  SortAccounts::entries_async(&db).with_key(&username.to_lowercase()).query_with_collection_docs().await?;
    if entry.len() == 0{
        anyhow::bail!("no matches")
    }
    ensure!(entry.len()==  1);
    let document = entry.into_iter().next().unwrap();
    if document.document.contents.password == password{
        // let token  = create_token_for(document.document.header.id).await?;
        let id_token= TargetIdAndToken{
            id: document.document.header.id,
            token : document.document.contents.auth_token
        };
        return Ok(id_token)
    }
    bail!("password incorrect")
    
}
pub async fn  getProfileById(id: u128)-> R<String>{

    let db = get_db().await?;
    let entry =  Account::get_async(&id, &db).await?.ok_or(anyhow!("error getting account from that id "))?;
    Ok(entry.contents.username)
}
