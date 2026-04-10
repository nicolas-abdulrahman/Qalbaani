use super::*;
use axum::body::{self, Body};
use axum::extract::{Path, Query};
use axum::http::{header, Method, Request, Response};
use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tracing_subscriber::EnvFilter;
use std::error::Error;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tower_http::{ services::{ServeDir, ServeFile}};

use chattyApi::*;

#[derive(Deserialize, Serialize)]
pub struct LoginData {
   pub  username: String,
    pub password : String,
}


#[derive(Serialize)]
pub struct MessageData{
    name : String,
    name_color : String,
    image : String,
    shadow_color : String,
    bubble_color : String,
}

#[derive(Serialize)]
pub struct UserData{
    name : String,
    name_color : String, 
    image_url: String,
}

pub async fn dummy2_get(Query(name): Query<LoginData> ) -> Json<LoginData>{
    println!("the query is: username {}, password: {}",name.username, name.password );
    Json(LoginData { username: "sunshinehappy".to_string(), password: "password12345".to_string() })
}

pub async fn dummy_get(  request : Request<body::Body>) -> Json<LoginData>{
    dbg!(request);
    Json(LoginData { username: "sunshinehappy".to_string(), password: "password12345".to_string() })
}
pub async fn signIn_api(Json(user) : Json<LoginData>) -> Result<Json<TargetIdAndToken>, (StatusCode, String)> {
    // let a = &*SERVER;
     match register_user(user.username, user.password).await{
       Ok(targetIdAndToken) => {return Ok(Json(targetIdAndToken))}
       Err(err) => {

        return Err((StatusCode::NOT_ACCEPTABLE, err.to_string()))
       }

    }
    // else{
        // return Err((StatusCode::NOT_ACCEPTABLE, "omg o.o".to_string()))
    // }

}
pub async fn root() -> &'static str {
    print!("FOUND A CONNECTION OMG");
    "Hello, World!"
}
// async fn serve_image() -> Router{
//
// }
pub async fn get_user_data() -> Json<UserData>{
    let user_data = UserData{
        name : "nick".to_string(), name_color: "#FFFFFF".to_string(), image_url : "http://192.168.10.9/assets/abcd/nick.png".to_string() 
    };
    Json(user_data)
    
}

//most stupid thing ever
trait Sexo{
   fn potato(&self);
}
impl Sexo for u128{
    fn potato(&self) {
        println!("i like {} potatoes", self);
    }
}

pub async fn add_friend_handler(Path((user_id, friend_id)): Path<(u128, String)>,
    token : String,
) {
        let token : u128 = token.parse().unwrap();
        add_friend(TargetIdAndToken{id: user_id, token}, friend_id).await;

}
pub async fn logIn_api(Json(user) : Json<LoginData>) -> Result<Json<TargetIdAndToken>, (StatusCode, String)> {
    match login(user.username, user.password).await{
    Ok(token) => {
        token.id.potato();
        return Ok(Json(token))
    }
       Err(err) => {

        return Err((StatusCode::NOT_ACCEPTABLE, err.to_string()))
       }

    }
}

pub async fn getProfileByIdHandler(Path(id): Path<u128>)-> Result<String, (StatusCode, String)>{
    match getProfileById(id).await{
        Ok(username) => {return Ok(username)}
        Err(err) => {return Err((StatusCode::NOT_ACCEPTABLE, err.to_string()))} 
    }
}

