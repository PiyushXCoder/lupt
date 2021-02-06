//! Chat Pinnd(पिण्ड) is Actor to manage Websocket Chat related action

use std::collections::HashMap;

use actix::{Actor, Addr, Context, Handler, Message};

use crate::ws_sansad;

#[allow(dead_code)]
pub struct ChatPinnd {
    grih: HashMap<i32, Grih>,
    vyaktigat_waitlist: Vec<Addr<ws_sansad::WsSansad>>
}

pub struct Grih {
    name: Option<String>,
    length: Option<usize>,
    clients: Vec<Addr<ws_sansad::WsSansad>>
}

// Handler Messages
pub struct Join {
    pub grih: JoinType,
    pub length: Option<usize>,
    pub addr: Addr<ws_sansad::WsSansad>
}

#[allow(dead_code)]
pub enum JoinType {
    Name(String),
    Kunjika(i32)
}

pub struct Text {
    pub grih_kunjika: i32,
    pub sender_name: String,
    pub text: String
}

pub struct  Delete {
    pub grih_kunjika: i32,
    pub addr: Addr<ws_sansad::WsSansad>
}

impl Actor for ChatPinnd {
    type Context = Context<Self>;
}

impl Handler<Join> for ChatPinnd {
    type Result = Option<i32>;

    fn handle(&mut self, msg: Join, _: &mut Self::Context) -> Self::Result {
        println!("Came to join");
        match msg.grih {
            JoinType::Name(name) => {
                let mat = Some(name.clone());
                if let Some((kunjika, grih)) = 
                    self.grih.iter_mut().find(|(_, g)| g.name == mat) {
                    if let Some(val) = grih.length {
                        if grih.clients.len()+1 == val {
                            return None;
                        }
                    }
                    grih.clients.push(msg.addr.clone());
                    return Some(*kunjika);
                }

                let mut kunjika: i32  = rand::random::<i32>();
                while self.grih.contains_key(&kunjika) {
                    kunjika = rand::random::<i32>();
                }
                println!("Creating {}", name);
                self.grih.insert(kunjika, Grih {
                    name: Some(name),
                    length: msg.length, 
                    clients: vec![msg.addr]
                });

                return Some(kunjika);
            }, JoinType::Kunjika(kunjika) => {
                match self.grih.get_mut(&kunjika) {
                    Some(grih) => {
                        if let Some(val) = grih.length {
                            if grih.clients.len()+1 == val {
                                return None;
                            }
                        }
                        grih.clients.push(msg.addr.clone());
                    }, None => {
                        self.grih.insert(kunjika, Grih {
                            name: None,
                            length: msg.length, 
                            clients: vec![msg.addr]
                        });
                    }
                }
                return Some(kunjika);
            }
        }
    }
}

impl Handler<Text> for ChatPinnd {
    type Result = ();

    fn handle(&mut self, msg: Text, _: &mut Self::Context) -> Self::Result {
        println!("Here to text");
        if let Some(grih) = self.grih.get(&msg.grih_kunjika) {
            for client in grih.clients.iter() {
                client.do_send(ws_sansad::Text {
                    sender: msg.sender_name.clone(),
                    text: msg.text.clone(),
                });
            }
        }
    }
}

impl Handler<Delete> for ChatPinnd {
    type Result = ();

    fn handle(&mut self, msg: Delete, _: &mut Self::Context) -> Self::Result {
        if let Some(grih) = self.grih.get_mut(&msg.grih_kunjika) {
            if let Some(i) = grih.clients.iter().position(|x| x == &msg.addr) {
                grih.clients.remove(i);
            }

            if grih.clients.len() == 0 {
                self.grih.remove(&msg.grih_kunjika);
            }
        }
    }
}

impl ChatPinnd {
    pub fn new() -> Self {
        ChatPinnd {
            grih: HashMap::new(),
            vyaktigat_waitlist: Vec::new()
        }
    }

    pub fn start() -> Addr<ChatPinnd> {
        ChatPinnd::new().start()
    }
}

impl Message for Join { type Result = Option<i32>; }
impl Message for Text { type Result = (); }
impl Message for Delete { type Result = (); }