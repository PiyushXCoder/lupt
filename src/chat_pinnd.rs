//! Chat Pinnd(पिण्ड) is Actor to manage Websocket Chat related action

use std::collections::HashMap;

use actix::prelude::*;
use actix_broker::BrokerSubscribe;

use crate::ws_sansad;
use crate::messages as ms;

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

impl Actor for ChatPinnd {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_system_async::<ms::ReciveText>(ctx);
        self.subscribe_system_async::<ms::LeaveUser>(ctx);
    }
}

impl Handler<ms::JoinUser> for ChatPinnd {
    type Result = Option<i32>;

    fn handle(&mut self, msg: ms::JoinUser, _: &mut Self::Context) -> Self::Result {
        match msg.grih {
            ms::JoinUserGrihType::Name(name) => {
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
                self.grih.insert(kunjika, Grih {
                    name: Some(name),
                    length: msg.length, 
                    clients: vec![msg.addr]
                });

                return Some(kunjika);
            }, ms::JoinUserGrihType::Kunjika(kunjika) => {
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

impl Handler<ms::ReciveText> for ChatPinnd {
    type Result = ();

    fn handle(&mut self, msg: ms::ReciveText, _: &mut Self::Context) -> Self::Result {
        println!("Here to text");
        if let Some(grih) = self.grih.get(&msg.grih_kunjika) {
            for client in grih.clients.iter() {
                client.do_send(ms::WsMessage {
                    sender: msg.sender_name.clone(),
                    text: msg.text.clone(),
                });
            }
        }
    }
}

impl Handler<ms::LeaveUser> for ChatPinnd {
    type Result = ();

    fn handle(&mut self, msg: ms::LeaveUser, _: &mut Self::Context) -> Self::Result {
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

impl Default for ChatPinnd {
    fn default() -> Self {
        ChatPinnd {
            grih: HashMap::new(),
            vyaktigat_waitlist: Vec::new()
        }
    }
}

impl SystemService for ChatPinnd {}
impl Supervised for ChatPinnd {}
