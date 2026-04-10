use std::sync::Mutex;


#[derive(Debug)]
pub struct Server{
    sessions: [Mutex<Session>; 10],
}


#[derive(Clone, Copy, Debug)]
pub struct Session{
    id : u8,
    status : SessionStatus,
    token : Option<u128>,
    stay_alive : bool, 
    duration: Option<i64>,
}

#[derive(Clone,Copy,Debug, PartialEq, Eq)]
pub enum  SessionStatus{
    Online,
    OnHold,
    Offline,
}

impl Server{
    fn create_session(id: u8)-> Mutex<Session>{
         Mutex::new(Session{id:id,
            status: SessionStatus::Offline, 
            token: None,
            stay_alive : true,
            duration : None})

        
    }
    pub fn create_server() -> Server{
        let session = Mutex::new(Session{id:0,
            status: SessionStatus::Offline, 
            token: None,
            stay_alive : true,
            duration : None});
        // let mut iter = 0..10;
        let sessions: [Mutex<Session>; 10] = core::array::from_fn(|a|{
                                                                  // let v = iter.next().unwrap(); 
                                                                    Server::create_session(a as u8)
        });
        // let sessions: [Mutex<Session>; 10]= (0..10).map(Server::create_session)
            // .collect();
        // let sessions_array: Box<[Mutex<Session>; 10]> =  sessions.into_boxed_slice().as_ref().try_into().unwrap();
        // for i in 0..10{
            // let id = i.clone() as u8;
            // sessions[i].id = id  
        // }
        Server{sessions}
    }

    pub fn take_offline_session(&self)-> Result<(u8, u128), anyhow::Error>{
        for session in &self.sessions{
            let mut session_l = session.try_lock();
            if session_l.is_err(){
                continue
            }
            let mut session_lock = session_l.unwrap();
            if  session_lock.status == SessionStatus::Offline{
                session_lock.status = SessionStatus::Online;
                session_lock.token = Some(uuid::Uuid::new_v4().as_u128());
                let id_token = (session_lock.id.clone(), session_lock.token.unwrap().clone());
                drop(session_lock);
                return Ok(id_token)
            }
        }
        anyhow::bail!("couldnt find empty server :O")

    }

    
}


#[test]
fn create_servers_test(){
    dbg!(Server::create_server());
}
