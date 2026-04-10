use std::time::SystemTime;

use anyhow::anyhow;
use bonsaidb::core::{schema::{Collection, Schema, View, ViewSchema, CollectionMapReduce, ViewMapResult, ReduceResult, ViewMappedValue}, connection::AsyncConnection, document::{CollectionDocument, Emit}, key::Key};
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Collection)]
#[collection(name = "messages", views=[GetMessageById, GetMessage], primary_key = AssociatedMessageKey)]
pub struct Message {
    #[natural_id]
    pub id: AssociatedMessageKey,
    pub timestamp: SystemTime,
    pub contents: String,
    pub profile : u64,

}


#[derive(Key, Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct AssociatedMessageKey {
    pub room_id: u32,
    pub chat_room_id: u64,
    pub message_id: u32,

}

#[derive(Debug, Serialize, Deserialize, Collection)]
#[collection(name = "accounts", primary_key = u128, views=[SortAccounts, SortAccountsById])]
pub struct Account {
    #[natural_id]
    pub id: u128,
    pub username : String,
    pub password : String,
    pub friends : Vec<u128>,
    pub chat_rooms : Vec<u64>,
    pub auth_token : u128,
    pub main_profile : u64,
    pub profiles :Vec<u64>,
    pub friend_requests: Vec<u128>, 


}


 

#[derive(Debug, Serialize, Deserialize, Collection)]
#[collection(name = "profiles" )]
pub struct Profile {
    pub name : String,
    pub name_color : String,
    pub image :String,
    pub bubble_color : String,
    pub status : String,
    pub owner : u128,
}
#[derive(Debug, Serialize, Deserialize, Collection)]
#[collection(name = "chat_rooms")]
pub struct ChatRoom {
    pub name : String,
    pub description : String, 
    pub chat_type : ChatType,
    pub rooms : Vec<u32>,
    pub users : Vec<u128>,
}
#[derive(Debug, Serialize, Deserialize, Collection)]
#[collection(name = "rooms", primary_key = AssociatedRoomKey)]
pub struct Room {
    #[natural_id]
    pub id : AssociatedRoomKey,
    pub name : String ,
    pub description : String,
    pub background : String,
    pub messages: u32
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ChatType{
    Private,
    GroupChat,
}

#[derive(Key, Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Deserialize, Serialize)]
pub struct AssociatedRoomKey {
    pub room_id: u32,
    pub chat_room_id: u64,
}


#[derive(Schema)]
#[schema(name = "ChatSchema", collections = [Room, Message])]
pub struct ChatSchema;

#[derive(Schema)]
#[schema(name = "MainSchema", collections = [Profile, Account, ChatRoom, Room, Message])]
pub struct MainSchema;

#[derive(Debug, Clone, View, ViewSchema)]
#[view(collection = Message, key = String, value = u64, name = "get_da_message2")]
pub struct GetMessage;

#[derive(Debug, Clone, View, ViewSchema)]
#[view(collection = Message, key = u32, value = u64, name = "get_da_message_byId3")]
pub struct GetMessageById;


#[derive(Debug, Clone, View, ViewSchema)]
#[view(collection = Account, key = String, value = u128, name = "sort_account")]
pub struct SortAccounts;

#[derive(Debug, Clone, View, ViewSchema)]
#[view(collection = Account, key = u128, value = u64, name = "sort_account_by_id")]
pub struct SortAccountsById;

impl CollectionMapReduce for GetMessage {
      fn map<'doc>(&self, document: CollectionDocument<Message>) -> ViewMapResult<'doc, Self::View>{
          document
            .header
            .emit_key_and_value(document.contents.contents,1)
    }

    fn reduce(
        &self,
        mappings: &[ViewMappedValue<'_, Self>],
        _rereduce: bool,
    ) -> ReduceResult<Self::View> {
        // Ok(1)
       Ok(mappings.iter().map(|mapping| mapping.value).sum())
    }
}


impl CollectionMapReduce for GetMessageById {
      fn map<'doc>(&self, document: CollectionDocument<Message>) -> ViewMapResult<'doc, Self::View>{
        // let post = Message::document_contents(document)?;
        document.header.emit_key_and_value(document.header.id.message_id, 1)
    }

    fn reduce(
        &self,
        mappings: &[ViewMappedValue<Self::View>],
        _rereduce: bool,
    ) -> ReduceResult<Self::View> {
        Ok(mappings.iter().map(|mapping| mapping.value).sum())
    }
}



impl CollectionMapReduce for SortAccounts {
      fn map<'doc>(&self, document: CollectionDocument<Account>) -> ViewMapResult<'doc, Self::View>{
          // dbg!(&document);
          document
            .header
            .emit_key_and_value(document.contents.username.to_lowercase(),document.header.id)
    }

    fn reduce(
        &self,
        mappings: &[ViewMappedValue<'_, Self>],
        _rereduce: bool,
    ) -> ReduceResult<Self::View> {
        // Ok(1)
        let closure = || -> Result<(), anyhow::Error> { Err(anyhow!("error in reduce")) };
       Ok(mappings.iter().map(|mapping| mapping.value).next().unwrap_or(0 as u128))
    }
}


impl CollectionMapReduce for SortAccountsById {
      fn map<'doc>(&self, document: CollectionDocument<Account>) -> ViewMapResult<'doc, Self::View>{
          document
            .header
            .emit_key_and_value(document.header.id,1)
    }

    fn reduce(
        &self,
        mappings: &[ViewMappedValue<'_, Self>],
        _rereduce: bool,
    ) -> ReduceResult<Self::View> {
        // Ok(1)
       Ok(mappings.iter().map(|mapping| mapping.value).sum())
    }
}


