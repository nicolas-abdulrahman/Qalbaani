#[allow(unused)]


#[cfg(test)]
mod client_test{
    use std::{net::{SocketAddr, SocketAddrV4, Ipv4Addr}, mem::MaybeUninit, sync::Arc, io::{Cursor, Read}, fs::File};

    use axum::Json;
    use futures::{SinkExt, StreamExt};
    use lazy_static::lazy_static;
    use regex::Regex;
    use serde::{Deserialize, Serialize};
    use anyhow::{Ok, ensure};
    use reqwest::{Client, StatusCode};
    use serde_json::{json, Value};
    use socket2::{Socket, Domain, Type, Protocol, SockAddr};
    use tokio::{net::{TcpSocket, TcpStream}, sync::{mpsc::channel, Mutex}, spawn};
    use tokio_tungstenite::{client_async, connect_async};
    use tungstenite::Message;
    use image::{io::Reader as ImageReader, codecs::png::PngEncoder, ImageOutputFormat};
    type R<T> = Result<T, anyhow::Error>;


    lazy_static!{

        static ref arr: [i32; 3] =[5, 12, 22];

    }
   #[tokio::test]
    async fn my_test()-> R<()>{
        // let arr = [5, 12, 22];
        arr.iter().map(move |v|{
            // async{
                println!("val {}", v);
            // }
        });
        // println!("hello {:?}", arr);
        Ok(())
    }

     #[tokio::test]
    async fn main_test()-> R<()>{
    // let server_addr = SocketAddr::from(([127,0,0,1], 3002));
    let server_addr: SocketAddr = "127.0.0.1:3002".parse()?;
    let socket: socket2::Socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))?;
    socket.set_nonblocking(false)?;
    socket
        .set_reuse_address(true)?;
    socket
        .set_reuse_port(true)?;
    println!("im heeeeeeeeeeeeeeeeeere");
    let sock_addr: SocketAddr= "127.0.0.1:3999".parse()?;
    socket.bind(&sock_addr.into())?;
    socket.connect(&server_addr.into())?;
    // let mut buf = [MaybeUninit::new(0); 10];
    let mut buf = [MaybeUninit::new(0 as u8); 50];
    let bytes = socket.recv(&mut buf)?;
    // println!("read {} bytes", bytes);

    // let sock_addr = ip
        // socket2::Socket::set_non
         // let stream = TcpStream::
         // stream.
        // println!("hello world");
        Ok(())
    } 
    // #[tokio::test]
    // async fn tcp_test()-> R<()>{
    //      let server_addr = "127.0.0.1:3002".parse().unwrap();
    //      let sock_addr = "127.0.0.1:3995".parse()?;
    //      TcpStream::ne
    // }

     
    #[tokio::test]
    async fn reqwest_test()-> R<()>{

        let url = "http://127.0.0.1:3002";
        let client = Client::new();

        let response = client.get(url).send().await?;
        println!("Status: {}", response.status());
         println!("Body: {:?}", response.text().await?);
         Ok(())

     
    } 

    #[derive(Debug, Serialize, Deserialize)]
    struct IdToken{
        id : u128,
        token : u128,
    }
    #[tokio::test]
    async fn reqwest_register_test_()-> R<()>{
        

        let url = "http://127.0.0.1:3002/sign_in";
        let client = Client::new();

        let payload = json!({
                "username" : "HappyySunshine",
                "password" : "ILikePonies12", 
        });

        let response = client.post(url).header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(&payload)?)
            .send().await?;
        println!("Status: {}", response.status());
         // println!("Body: {:?}", response.text().await?);
        let res: IdToken= serde_json::from_str(&response.text().await?)?;
        println!("id :{} token: {}", res.id, res.token);
        
        let payload = json!({
                "username" : "HayaDaCuteBun",
                "password" : "ILikePonies123", 
        });

        let response = client.post(url).header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(&payload)?)
            .send().await?;
         Ok(())

     
    }
    #[tokio::test]
    async fn reqwest_add_friend_id_test_()-> R<()>{
        


        let url = "http://127.0.0.1:3002/sign_in";
        let client = Client::new();

        let payload = json!({ 
                 "username" : "sunnyIsCool3",
                "password" : "ILikePonies12", 
        });

        let response = client.post(url).header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(&payload)?)
            .send().await?; 
        // println!("Status: {}", response.status());
         // println!("Body: {:?}", response.text().await?);
        let res: IdToken= serde_json::from_str(&response.text().await?)?;
         let payload = json!({
                "username" : "HayaIsCute3",
                "password" : "ILikePonies12", 
        });

        let response = client.post(url).header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(&payload)?)
            .send().await?; 
         // println!("Body: {:?}", response.text().await?);
        let res2: IdToken= serde_json::from_str(&response.text().await?)?;

        println!("id1 :{} token1: {}", res.id, res.token);
        println!("id2 :{} token2: {}", res2.id, res2.token);

        let url = format!("http://127.0.0.1:3002/add_friend/{}/{}", res.id, res2.id);
        let response = client.post(url).header(reqwest::header::CONTENT_TYPE, "text/plain")
            .body(res.token.to_string())
            .send().await?; 

        
        println!("Status: {}", response.status());


         Ok(())

     
    }  
    // #[tokio::test]
    // async fn reqwest_add_friend_id_test_()-> R<()>{
    //    
    //
    //g;//
    //     let url = "ws://127.0.0.1:3002/ws";
    //     let client = Client::new();
    //
    //
    //
    // }
    #[derive(Deserialize, Debug)]
    struct ServerMessage{
        pub chat_room : u64, 
        pub friend_id: u128,
    }
    #[tokio::test]
    async fn tung_test()-> R<()>{
        // let server_addr = "127.0.0.1:3002".parse().unwrap();
        // let sock_addr = "127.0.0.1:3995".parse()?;
        
        
        let client = Client::new();
        let url = "http://127.0.0.1:3002/log_in";
        let payload = json!({
                "username" : "HappyySunshine",
                "password" : "ILikePonies12", 
        });

        let response = client.post(url).header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(&payload)?)
            .send().await?; 
        println!("{}",response.status());
        let text= response.text().await?;
        println!("{}",text.clone());
        let id_token: IdToken = serde_json::from_str(&text)?;
        dbg!(&id_token);
        // let url = url::Url::parse(
                    // format!("ws://127.0.0.1:3002/ws?id={}&token={}",id_token.id, id_token.token).as_str()
                    // )?;
        dbg!(&url);


        let url = format!("ws://127.0.0.1:3002/ws?id={}&token={}", id_token.id, id_token.token);

        // assert_eq!(response.status(), StatusCode::OK);
        let (mut socket , _response) = connect_async(url).await?;

        let (mut sen, mut rec)= socket.split();
        let sender = Arc::new(Mutex::new(sen));
        // send.send()
        let sender_c = sender.clone();

        let thread = spawn(async move{
            loop{
            let next = rec.next().await.unwrap().unwrap();
                let message = next.to_text().unwrap();
                println!("message from web SOCKET: {}", message);
                let m = message.clone();
                let first_char =message.chars().next();
                if first_char.is_some(){
                    if first_char.unwrap()=='{'{
                        let mess:ServerMessage = serde_json::from_str(message).unwrap();
                        println!("got some important info xd {:?}", &mess);
                        let payload = json!({
                            "T": "sm",
                            "to_chat_room": mess.chat_room,
                            "data": "hello bob :D",
                        });
                        sender_c.lock().await.send(Message::Text(payload.to_string())).await.unwrap();
                        let payload = json!({
                            "T": "sm",
                            "to_chat_room": mess.chat_room,
                            "data": "how are u doing bob?",
                        });
                        sender_c.lock().await.send(Message::Text(payload.to_string())).await.unwrap();
                        let payload = json!({
                            "T": "sm",
                            "to_chat_room": mess.chat_room,
                            "data": "i miss you bobby?",
                        });
                        sender_c.lock().await.send(Message::Text(payload.to_string())).await.unwrap();

                     let payload = json!({
                            "T": "gm",
                            "to_chat_room": mess.chat_room,
                            "to_room":0,
                            "data": "0",
                        });
                        sender_c.lock().await.send(Message::Text(payload.to_string())).await.unwrap();

                    }
                }
            }
            
        });
           
        // println!("message from web SOCKET is {}", m.to_text()?);
        let son = json!({
                  "T": "af",    
                  "data": "HayaDaCuteBun"
            });
        sender.lock().await.send(Message::Text(son.to_string())).await?;
        let name = "cuddles.png".to_string();
        let mut image = ImageReader::open(&name)?;
        image.set_format(image::ImageFormat::Png);
        let image = image.decode()?;
        let mut bytes: Vec<u8> = Vec::new();
        image.write_to(&mut Cursor::new(&mut bytes),ImageOutputFormat::Png)?;

        let mut file = File::open("cuddles.png")?;
    // Read all bytes from the file into a Vec<u8>
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;

        // bytes.push(254 as u8);
        // let string = String::from_utf8(bytes)?;
        // let mut image = ImageReader::new(Cursor::new(bytes));
        // image.with_guessed_format();
        // let image = image.with_guessed_format()?.decode()?;
        // image.save("testing2.png");
        #[derive(Serialize)]
        struct ImageMessage{
            pub name: String,
            pub bytes: Vec<u8>,
        }
        let img_to_send = ImageMessage{name, bytes};
        let data = serde_json::to_string(&img_to_send)?;
        let payload = json!({
            "T": "ui",
            "data": data,
        });
        sender.lock().await.send(Message::Text(serde_json::to_string(&payload)?)).await?;
        // let encoded = image.write_with_encoder(PngEncoder)?;
         
        // image.save("testing.png");

        thread.await?;
        // let res = socket.read()?;
        // println!("message from web SOCKET is {}", res.to_text()?);

        
        Ok(())
    }
    #[tokio::test]
    async fn test_tokio()-> R<()>{
         let server_addr = "127.0.0.1:3002".parse().unwrap();
         let sock_addr = "127.0.0.1:3995".parse()?;
    // let ws_stream =     TcpStream::connect(sock_addr)?;
    let socket = TcpSocket::new_v4()?;
    socket.bind(sock_addr)?;
    //
    println!("connection happening..");
    let stream = socket.connect(server_addr).await?; 
    // let mut buf = [0 as u8; 50];

    // let bytes =  stream.try_read(&mut buf)?;
    // println!("read {} bytes", bytes);
    Ok(())

    }


    #[tokio::test]
    async fn raw_socket_test()-> R<()>{
        
         // let server_addr: SocketAddr = "127.0.0.1:3002".parse()?;
        let socket: socket2::Socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))?;
        socket.set_nonblocking(false)?;
        socket
            .set_reuse_address(true)?;
        socket
            .set_reuse_port(true)?;
         let server_addr =SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 3002);
         let sock_addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 4000);
            socket
            .bind(&sock_addr.into())?;

        socket.connect(&server_addr.into())?;
        // let socket22 = socket.try_clone()?;
        // sock
         let message = String::from("GET /dummy HTTP/1.1\r
Host: your mom\r
Connection: keep-alive\r\n\r\n
");

        let rest = socket.send(message.as_bytes())?;
        let mut r = [MaybeUninit::<u8>::uninit();400];
        let response = socket.recv(&mut r);   
        // assume_init();
        let string = r.iter().map(|v| {
            let val = unsafe {v.assume_init()};
            return val as char 

        }).collect::<String>();
        print!("--- here is the first ---");
        dbg!(string);
let message = String::from("GET /dummy2?username=bob&password=password123 HTTP/1.1\r
Host: your mom_is_gay\r
Connection: keep-alive\r\n\r\n
");

        let rest = socket.send(message.as_bytes())?;
        let mut r = [MaybeUninit::<u8>::uninit();400];
        let response = socket.recv(&mut r);   
        // assume_init();
        let string = r.iter().map(|v| {
            let val = unsafe {v.assume_init()};
            return val as char 

        }).collect::<String>();
        print!("--- here is the first ---");
        dbg!(string);
        let message = String::from("POST /sign_in HTTP/1.1\r
Host: 127.0.0.1:3002\r
Connection: keep-alive\r
Content-Type: application/json\r
content-length: 56\r
\r\n{\"username\":\"sunshinehappyy\",\"password\":\"password12345\"}");
        let rest = socket.send(message.as_bytes())?;
        let mut r = [MaybeUninit::<u8>::uninit();600];
        let response = socket.recv(&mut r);   
        // assume_init();
        let string = r.iter().map(|v| {
            let val = unsafe {v.assume_init()};
            return val as char 

        }).collect::<String>();
        dbg!(string);
        // print!("{:?}", unsafe{ assume_ini})
        print!("connection done.");

        Ok(())

    }
    
    #[tokio::test]
    async fn raw_websocket_test()-> R<()>{
        
         // let server_addr: SocketAddr = "127.0.0.1:3002".parse()?;
        let socket: socket2::Socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))?;
        socket.set_nonblocking(false)?;
        socket
            .set_reuse_address(true)?;
        socket
            .set_reuse_port(true)?;
         let server_addr =SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 3002);
         let sock_addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 4000);
            socket
            .bind(&sock_addr.into())?;
          socket.connect(&server_addr.into())?;
        let content =  "{\"username\":\"sunshinehappyy\",\"password\":\"password12345\"}";
        let header= String::from(format!("POST /log_in HTTP/1.1\r
Host: 127.0.0.1:3002\r
Connection: keep-alive\r
Content-Type: application/json\r
content-length: {}\r\n\r\n", content.len()));
     let message = header + content;
        let rest = socket.send(message.as_bytes())?;
        let mut r = [MaybeUninit::<u8>::uninit();600];
        let response = socket.recv(&mut r); 
        
        // assume_init();
        let string = r.iter().map(|v| {
            let val = unsafe {v.assume_init()};
            return val as char 

        }).collect::<String>();
        let pattern = Regex::new(r"\{.*?\}").unwrap();
        // dbg!(&string);
        let matchh = pattern.find(string.as_ref()).unwrap();
        let id_token: IdToken = serde_json::from_str(matchh.as_str())?;



        // dbg!(matchh);
        println!("{:?}",id_token);

        let message = String::from(format!("GET /ws?id={}&token={} HTTP/1.1\r
Host: my mom is gay\r
Connection: Upgrade\r
upgrade: websocket\r
sec-websocket-version: 13\r
sec-websocket-key: yvrMTMV0/mFYGHOsFPWfqg==\r\n\r\n",id_token.id, id_token.token));
        let rest = socket.send(message.as_bytes())?;
        let mut r = [MaybeUninit::<u8>::uninit();600];
        let response = socket.recv(&mut r);   
        // assume_init();
        let string = r.iter().map(|v| {
            let val = unsafe {v.assume_init()};
            return val as char 

        }).collect::<String>();
        dbg!(string);
        
        // print!("{:?}", unsafe{ assume_ini})
        let mut r = [MaybeUninit::<u8>::uninit();600];
        let response = socket.recv(&mut r);
        
        // assume_init();
        let string = r.iter().map(|v| {
            let val = unsafe {v.assume_init()};
            return val as char 

        }).collect::<String>();
        // let m = Message::from(string);
        println!("received {}", &string);
        dbg!(string);
        let mut r = [MaybeUninit::<u8>::uninit();600];
        let response = socket.recv(&mut r);   
        // assume_init();
        let string = r.iter().map(|v| {
            let val = unsafe {v.assume_init()};
            return val as char 

        }).collect::<String>();
        println!("received {}", string);

        print!("connection done.");



        // socket.connect(&server_addr.into())?;
        Ok(())
    }


}
