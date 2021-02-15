//! Chat Pinnd(पिण्ड) is Actor to manage Websocket Chat related action

use std::collections::HashMap;

use actix::prelude::*;
use actix_broker::BrokerSubscribe;
use ws_sansad::WsSansad;

use crate::{errors, ws_sansad, messages as ms};

#[allow(dead_code)]
pub struct ChatPinnd {
    grih: HashMap<String, Grih>, // id, Grih
    vyaktigat_waitlist: Vec<Addr<ws_sansad::WsSansad>>,
    vyakti: Vec<Vyakti> // id, tags
}

pub struct Grih {
    length: Option<usize>,
    loog: Vec<Addr<ws_sansad::WsSansad>>
}

pub struct Vyakti {
    kunjika: String,
    tags: Vec<String>
}

impl Actor for ChatPinnd {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_system_async::<ms::SendText>(ctx);
        self.subscribe_system_async::<ms::LeaveUser>(ctx);
    }
}

impl Handler<ms::SetKunjikaUser> for ChatPinnd {
    type Result = Result<(), errors::AlreadyExistError>;

    fn handle(&mut self, msg: ms::SetKunjikaUser, ctx: &mut Self::Context) -> Self::Result {
        let kunjika = msg.kunjika;
        let vyakti = self.vyakti.iter().find(|a| a.kunjika == kunjika);
        Ok(())
    }
}

impl Handler<ms::JoinUser> for ChatPinnd {
    type Result = Result<(), errors::GrihFullError>;

    fn handle(&mut self, msg: ms::JoinUser, _: &mut Self::Context) -> Self::Result {
        match self.grih.get_mut(&msg.grih_kunjika) {
            Some(grih) =>{
                if let Some(n) = grih.length {
                    println!("length check {}, {}, {}", grih.loog.len(), n,grih.loog.len() >= n);
                    if grih.loog.len() >= n {
                        return Err(errors::GrihFullError);
                    } 
                }

                grih.loog.push(msg.addr);
                let username = msg.name.clone();
                grih.loog.iter().for_each(move |a: &Addr<WsSansad>| {
                    a.do_send(ms::WsConnected {
                        user: username.clone()
                    })
                });
            }, None => {
                self.grih.insert(msg.grih_kunjika, Grih {
                    length: msg.length,
                    loog: vec![msg.addr.clone()]
                });
                msg.addr.do_send(ms::WsConnected {
                    user: msg.name
                });
            }
        }

        Ok(())
    }
}

impl Handler<ms::SendText> for ChatPinnd {
    type Result = ();

    fn handle(&mut self, msg: ms::SendText, _: &mut Self::Context) -> Self::Result {
        if let Some(grih) = self.grih.get(&msg.grih_kunjika) {
            grih.loog.iter().for_each(|c| {
                c.do_send(ms::WsMessage {
                    sender: msg.sender_name.clone(),
                    text: msg.text.clone(),
                });
            });
        }
    }
}

impl Handler<ms::LeaveUser> for ChatPinnd {
    type Result = ();

    fn handle(&mut self, msg: ms::LeaveUser, _: &mut Self::Context) -> Self::Result {
        if let Some(grih) = self.grih.get_mut(&msg.grih_kunjika) {
            if let Some(i) = grih.loog.iter().position(|x| x == &msg.addr) {
                grih.loog.remove(i);
            }

            if grih.loog.len() == 0 {
                self.grih.remove(&msg.grih_kunjika);
            } else {
                grih.loog.iter().for_each(|a| {
                    a.do_send(ms::WsDisconnected {
                        user: "u".to_owned()
                    })
                });
            }
        }
    }
}

impl Default for ChatPinnd {
    fn default() -> Self {
        ChatPinnd {
            grih: HashMap::new(),
            vyaktigat_waitlist: Vec::new(),
            vyakti: Vec::new()
        }
    }
}

impl SystemService for ChatPinnd {}
impl Supervised for ChatPinnd {}
