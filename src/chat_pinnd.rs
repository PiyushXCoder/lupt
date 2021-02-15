//! Chat Pinnd(पिण्ड) is Actor to manage Websocket Chat related action

use std::collections::HashMap;

use actix::prelude::*;
use actix_broker::BrokerSubscribe;
use vecmap::VecMap;
use ws_sansad::WsSansad;

use crate::{errors, ws_sansad, messages as ms};

#[allow(dead_code)]
pub struct ChatPinnd {
    grih: HashMap<String, Grih>, // id, Grih
    vyaktigat_waitlist: Vec<VyaktiWatchlist>,
    vyakti: VecMap<String, Vyakti> // id, vayakti
}

pub struct Grih {
    length: Option<usize>,
    loog: Vec<Addr<ws_sansad::WsSansad>>
}

#[derive(Clone)]
pub struct Vyakti {
    name: String,
    tags: Vec<String>
}

pub struct VyaktiWatchlist {
    kunjika: String,
    addr: Addr<ws_sansad::WsSansad>
}

impl Actor for ChatPinnd {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_system_async::<ms::SendText>(ctx);
        self.subscribe_system_async::<ms::LeaveUser>(ctx);
    }
}

impl Handler<ms::SetInfoVyakti> for ChatPinnd {
    type Result = Option<String>;

    fn handle(&mut self, msg: ms::SetInfoVyakti, _: &mut Self::Context) -> Self::Result {
        if !msg.modify {
            if self.vyakti.key_exist(&msg.kunjika) {
                return Some("Kunjika Exists".to_owned());
            }
        }

        self.vyakti.insert(msg.kunjika, Vyakti {
            name: msg.name,
            tags: msg.tags
        });

        None
    }
}

impl Handler<ms::JoinGrih> for ChatPinnd {
    type Result = Result<(), errors::GrihFullError>;

    fn handle(&mut self, msg: ms::JoinGrih, _: &mut Self::Context) -> Self::Result {
        match self.grih.get_mut(&msg.grih_kunjika) {
            Some(grih) =>{
                if let Some(n) = grih.length {
                    if grih.loog.len() >= n {
                        return Err(errors::GrihFullError);
                    } 
                }

                grih.loog.push(msg.addr);
                let username = self.vyakti.get(&msg.kunjika).unwrap().name.to_owned();
                let kunjika = msg.kunjika.clone();
                grih.loog.iter().for_each(move |a: &Addr<WsSansad>| {
                    a.do_send(ms::WsConnected {
                        name: username.clone(),
                        kunjika: kunjika.clone()
                    })
                });
            }, None => {
                self.grih.insert(msg.grih_kunjika, Grih {
                    length: msg.length,
                    loog: vec![msg.addr.clone()]
                });
                msg.addr.do_send(ms::WsConnected {
                    name: self.vyakti.get(&msg.kunjika).unwrap().name.to_owned(),
                    kunjika: msg.kunjika.clone()
                });
            }
        }

        Ok(())
    }
}

impl Handler<ms::JoinRandom> for ChatPinnd {
    type Result = Option<()>;
    fn handle(&mut self, msg: ms::JoinRandom, _: &mut Self::Context) -> Self::Result {
        if self.vyaktigat_waitlist.len() == 0 {
            self.vyaktigat_waitlist.push(VyaktiWatchlist {
                kunjika: msg.kunjika,
                addr: msg.addr
            });
            return None;
        }

        let vayakti_watchlist = self.vyaktigat_waitlist.remove(0);
        let vayakti1 =  self.vyakti.get(&msg.kunjika).unwrap();
        let vayakti2 = self.vyakti.get(&vayakti_watchlist.kunjika).unwrap();
        let group_kunjika = format!("gupt_{}>{}",msg.kunjika, vayakti_watchlist.kunjika);
        self.grih.insert(group_kunjika.clone(), Grih {
            length: Some(2),
            loog: vec![msg.addr.clone(), vayakti_watchlist.addr.clone()]
        });
        
        msg.addr.do_send(ms::WsConnectedRandom {
            ajnyat_name: vayakti2.name.clone(),
            grih_kunjika: group_kunjika.clone()
        });

        vayakti_watchlist.addr.do_send(ms::WsConnectedRandom {
            ajnyat_name: vayakti1.name.clone(),
            grih_kunjika: group_kunjika.clone()
        });

        Some(())
    }
}

impl Handler<ms::SendText> for ChatPinnd {
    type Result = ();

    fn handle(&mut self, msg: ms::SendText, _: &mut Self::Context) -> Self::Result {
        if let Some(grih) = self.grih.get(&msg.grih_kunjika) {
            grih.loog.iter().for_each(|c| {
                c.do_send(ms::WsMessage {
                    sender: msg.kunjika.clone(),
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
                        kunjika: msg.kunjika.clone()
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
            vyakti: VecMap::new()
        }
    }
}

impl SystemService for ChatPinnd {}
impl Supervised for ChatPinnd {}
