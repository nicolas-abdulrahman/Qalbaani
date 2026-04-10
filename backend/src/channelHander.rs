use std::{sync::Arc, net::SocketAddr};

use axum::extract::ws::{WebSocket, Message};
use futures::{stream::SplitSink, SinkExt};
use serde::{Serialize, Deserialize};
use tokio::sync::{Mutex, mpsc::Sender};

use crate::R;


#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum MessageType{
    #[serde(rename="ac")]
    add_connection,
    #[serde(rename="sm")]
    send_message,
    #[serde(rename="cs")]
    close_socket,
    #[serde(rename="af")]
    add_friend,
    #[serde(rename="gm")]
    get_messages,
    #[serde(rename="ui")]
    upload_image,
    
}
#[derive(Debug, Clone)]
pub struct ChannelMessage{
    pub message_type : MessageType,
    pub ws_sender : Option<Arc<Mutex<SplitSink<WebSocket, Message>>>>,
    // pub ws_sender : Sender<String>,
    pub from : SocketAddr,
    pub from_id : u128,
    pub to_chat_room : Option<u64>,
    pub to_room : Option<u32>,
    pub token : u128,
    pub data : String, 
}
#[derive(Serialize, Deserialize, Debug)]
pub struct IncomingMessage {
    #[serde(rename="T")]
    message_type : MessageType,
    from_id: Option<u128>,
    to_chat_room: Option<u64>,
    token :Option<u128>,
    to_room : Option<u32>,
    with_profile : Option<u64>,
    data: String,
}
#[derive(Deserialize, Debug)]
pub struct ImageMessage{
    pub name: String,
    pub bytes: Vec<u8>
}

#[derive(Debug, Clone)]
pub struct ChannelHandler{
    pub sender :  Sender<ChannelMessage>,
    pub ws_sender : Arc<Mutex<SplitSink<WebSocket, Message>>>,
    pub from : SocketAddr,
    pub from_id : u128,
    pub token : u128,
}

impl From<&ChannelHandler> for ChannelMessage{
    fn from(item: &ChannelHandler) -> Self {
        ChannelMessage {
            message_type: MessageType::add_connection, 
            ws_sender: Some(item.ws_sender.clone()), 
            from: item.from, 
            from_id: item.from_id,
            to_chat_room: None, 
            to_room: None, 
            token: item.token,
            data: "".to_string() }
    }

}
impl IncomingMessage{
    fn add_channel_message_info(self,channel : &ChannelHandler)-> ChannelMessage {
        ChannelMessage{
            message_type: self.message_type, 
            ws_sender: Some(channel.ws_sender.clone()), 
            from: channel.from, 
            from_id: channel.from_id,
            to_chat_room: self.to_chat_room,
            to_room: self.to_room, 
            token: channel.token,
            data: self.data,

        }
        // self.ws_sender = Some(channel.ws_sender.clone());
        // self.from_id = channel.from_id; 
        // self.from = channel.from;
        // self.token = channel.token;
        // return self


    }
}
impl ChannelHandler{
    pub async fn send_start_connection(&mut self)-> R<()>{
        let message = ChannelMessage::from(&*self);
        self.sender.send(message).await?;
        Ok(())
    }

    pub async fn send_ws_text(&mut self, text: String)->  R<()>{
        self.ws_sender.lock().await.send(Message::Text(text)).await?;
        self.ws_sender.lock().await.flush().await?;

        // self.ws_sender.lock().await.;
        println!("sent message!");

        
        Ok(())
    }

    pub async fn send_from_incoming_message(&mut self, incomming_message: IncomingMessage)-> R<()>{
        // let mut message = ChannelMessage::from(&*self);
        let message = incomming_message.add_channel_message_info(self);
        // println!("DA MESSAGE TO SERVER IS {:?}", message);
        // message.message_type = incomming_message.message_type;
        // message.data = incomming_message.data;
        self.sender.send(message).await?;

        Ok(())
    }

    pub async fn send_close(&self)-> R<()>{
        let mut message = ChannelMessage::from(self);
        message.message_type = MessageType::close_socket;
        self.sender.send(message).await?;
        Ok(())
    }
}
