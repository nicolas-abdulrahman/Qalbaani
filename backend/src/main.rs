use axum::http::{header, Method, Request, Response};
use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use chattyApi::channelHander::ChannelMessage;
use chattyApi::server::*;
use chattyApi::{ TargetIdAndToken, register_user, create_database, create_default_profile};
// use crate::channelHandler;

use futures::task::ArcWake;
use serde::{Deserialize, Serialize};
use tokio::spawn;
use tokio::sync::Mutex;
use tokio::sync::mpsc::{channel, Sender, Receiver};
use tracing_subscriber::EnvFilter;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::{ services::{ServeDir, ServeFile}};

use chattyApi::R;
#[macro_use]
extern crate lazy_static;



struct SenderReceiver{
    RECEIVER: Arc<Mutex<Receiver<ChannelMessage>>>,
    SENDER: Sender<ChannelMessage>,
}

lazy_static!{
    // static ref SERVER : chattyApi::server::Server= Server::create_server();
    static ref STATIC: SenderReceiver = {
        let (sender, receiver) = channel::<ChannelMessage>(100);
        // Spawn a separate thread for receiving

        SenderReceiver{
            RECEIVER : Arc::new(Mutex::new(receiver)),
            SENDER : sender,
        }

        

        
        
        // sender
    };
}
mod path;
use path::*;

mod ws;
use ws::*;


//mod ws;
//use ws::ws_handler;
#[tokio::main]
async fn main() {
       
    // tracing_subscriber::registry()
    //     .with(
    //         tracing_subscriber::EnvFilter::try_from_default_env()
    //             .unwrap_or_else(|_| "example_websockets=debug,tower_http=debug".into()),
    //     )
    //     .with(tracing_subscriber::fmt::layer())
    //     .init();
    // initialize tracing
    println!("hello");
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    create_database().await.unwrap();


     let mut file = File::create("log").unwrap();
     drop(file);

    
    let mut server = Server{
        users_connected : Mutex::new(HashMap::new()),
        rooms : Mutex::new(HashMap::new()),
        receiver : STATIC.RECEIVER.clone(),

    };
    let server_handle = spawn(async move {

        let  serv = Arc::new(server);
        serv.start_server().await;
    });
    // STATIC.SERVER.start_server();

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/home", get(root))
        .route("/dummy", get(dummy_get))
        .route("/dummy2", get(dummy2_get))

        .route("/ws", get(ws_handler) )
        .route("/add_friend/:user_id/:friend_id", post(add_friend_handler))
        .route("/sign_in", post(signIn_api))
        .route("/log_in", post(logIn_api))
        .route("/getUser/:id", get(getProfileByIdHandler))
        .route("/user/sunshine", get(get_user_data))
        .nest("/assets", axum_static::static_router("assets"))
        // `POST /users` goes to `create_user`
        .layer(cors);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    // let addr = SocketAddr::from(([192,168,10,9], 80));
    
    let addr = SocketAddr::from(([127,0,0,1], 3002));
    tracing::debug!("listening on {}", addr);
    println!("running at {:?}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
    println!("running at {:?}", &addr);
 }

fn spawn_server(){

}

#[cfg(test)]
mod test;
//
//     use chattyApi::create_database;
//
//     use super::*;
//     #[tokio::test]
//     async fn main_test()-> R<()> {
//         create_database().await?;
//         let a = register_user( "happy".to_string(), "password12345".to_string()).await?;
//         dbg!(a); 
//         let b = register_user( "hay hay".to_string(), "password12345".to_string()).await?;
//         dbg!(b);
//
//         anyhow::ensure!(register_user("happy".to_string(), "password12345".to_string()).await.is_err());
//         Ok(())
//     }
//     // #[tokio::test]
//     // async fn start_test() {
//     //     // dbg!(&*SERVER);
//     // }
// }



