//! Chat Pinnd(पिण्ड) is Actor to manage Websocket Chat related action

use std::{collections::HashMap, vec};

use actix::prelude::*;
use actix_broker::BrokerSubscribe;
use ms::Resp;

use crate::{ws_sansad, messages as ms};

#[allow(dead_code)]
pub struct ChatPinnd {
    grih: HashMap<String, Grih>, // kunjika, Grih
    vyaktigat_waitlist: Vec<VyaktiWatchlist>,
}

pub struct Grih {
    length: Option<usize>,
    loog: Vec<Loog>
}

pub struct Loog {
    addr: Addr<ws_sansad::WsSansad>,
    kunjika: String,
    name: String,
    tags: Option<Vec<String>>
}

#[derive(Debug, Clone)]
pub struct Vyakti {
    name: String,
    tags: Vec<String>
}
pub struct VyaktiWatchlist {
    kunjika: String,
    name: String,
    tags: Vec<String>,
    addr: Addr<ws_sansad::WsSansad>
}

impl Actor for ChatPinnd {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // for actix broker 
        self.subscribe_system_async::<ms::SendText>(ctx);
        self.subscribe_system_async::<ms::SendStatus>(ctx);
        self.subscribe_system_async::<ms::LeaveUser>(ctx);
    }
}

/// Join grih
impl Handler<ms::JoinGrih> for ChatPinnd {
    type Result = Resp;

    fn handle(&mut self, msg: ms::JoinGrih, _: &mut Self::Context) -> Self::Result {
        // check if user exist
        if let Some(_) = self.vyaktigat_waitlist.iter().position(|vk| vk.kunjika == msg.kunjika) {
            return Resp::Err("Kunjika already exist".to_owned());
        }

        if let Some(_) = self.grih.iter().position(|(_,g)| {
            match g.loog.iter().position(|a| a.kunjika == msg.kunjika) {
                Some(_) => true,
                None => false
            }
        }) {
            return Resp::Err("Kunjika already exist".to_owned());
        }

        // check if grih exist and add user
        match self.grih.get_mut(&msg.grih_kunjika) { 
            Some(grih) =>{ // exist
                // check if grih have no space left
                if let Some(n) = grih.length {
                    if grih.loog.len() >= n {
                        return Resp::Err("Grih have no space".to_owned());
                    } 
                }

                grih.loog.iter().for_each(|a: &Loog| {
                    a.addr.do_send(ms::WsConnected {
                        name: msg.name.to_owned(),
                        kunjika: msg.kunjika.to_owned()
                    })
                });

                grih.loog.push(Loog::new(msg.addr, msg.kunjika,msg.name, None));

                
            }, None => { // don't exist
                // add grih and notify
                msg.addr.do_send(ms::WsConnected {
                    name: msg.name.to_owned(),
                    kunjika: msg.kunjika.to_owned()
                });
                self.grih.insert(msg.grih_kunjika, Grih {
                    length: msg.length,
                    loog: vec![Loog::new(msg.addr,msg.kunjika,msg.name, None)]
                });
            }
        }

        Resp::Ok
    }
}

/// Join random vayakti 
/// Works as:
/// Check if watchlist is empty, if yes add the kunjika andaddr to watchlist
/// if watchlist have people get 0th  person an connect it
impl Handler<ms::JoinRandom> for ChatPinnd {
    type Result = Resp;
    fn handle(&mut self, msg: ms::JoinRandom, _: &mut Self::Context) -> Self::Result {
        // check if user exist
        if let Some(_) = self.vyaktigat_waitlist.iter().position(|vk| vk.kunjika == msg.kunjika) {
            return Resp::Err("Kunjika already exist".to_owned());
        }

        if let Some(_) = self.grih.iter().position(|(_,g)| {
            match g.loog.iter().position(|a| a.kunjika == msg.kunjika) {
                Some(_) => true,
                None => false
            }
        }) {
            return Resp::Err("Kunjika already exist".to_owned());
        }

        // Check if watch list is empty
        if self.vyaktigat_waitlist.len() == 0 {
            self.vyaktigat_waitlist.push(VyaktiWatchlist {
                kunjika: msg.kunjika,
                addr: msg.addr,
                name: msg.name,
                tags: msg.tags
            });
            return Resp::None;
        }
        
        // connect person with tag 
        let pos = if msg.tags.len() > 0 {
            match self.vyaktigat_waitlist.iter().position(|vk| {
                match vk.tags.iter().position(|t| msg.tags.contains(t)) {
                    Some(_) => true,
                    None => false
                }
            }) {
                Some(i) => i,
                None => {
                    self.vyaktigat_waitlist.push(VyaktiWatchlist {
                        kunjika: msg.kunjika,
                        addr: msg.addr,
                        name: msg.name,
                        tags: msg.tags
                    });
                    return Resp::None;
                }
            }
        } else { 0 };

        let vayakti_watchlist = self.vyaktigat_waitlist.remove(pos);
        let group_kunjika = format!("gupt_{}>{}",msg.kunjika.to_owned(), vayakti_watchlist.kunjika);
        self.grih.insert(group_kunjika.to_owned(), Grih {
            length: Some(2),
            loog: vec![Loog::new(msg.addr.clone(), msg.kunjika.to_owned(), msg.name.to_owned(), Some(msg.tags.clone())),
                Loog::new(vayakti_watchlist.addr.clone(), vayakti_watchlist.kunjika.to_owned(), vayakti_watchlist.name.to_owned(), Some(vayakti_watchlist.tags.clone()))]
        });
        
        // notify about connection
        msg.addr.do_send(ms::WsConnectedRandom {
            name: vayakti_watchlist.name,
            kunjika: vayakti_watchlist.kunjika,
            grih_kunjika: group_kunjika.to_owned()
        });
        vayakti_watchlist.addr.do_send(ms::WsConnectedRandom {
            name: msg.name,
            kunjika: msg.kunjika.to_owned(),
            grih_kunjika: group_kunjika
        });

        Resp::Ok
    }
}

/// Next Random user
impl Handler<ms::JoinRandomNext> for ChatPinnd {
    type Result = Resp;
    fn handle(&mut self, msg: ms::JoinRandomNext, _: &mut Self::Context) -> Self::Result {
        let grih  = self.grih.get_mut(&msg.grih_kunjika).unwrap();

        let loog_i = grih.loog.iter().position(|a| a.kunjika == msg.kunjika).unwrap();
        
        let addr;
        let name;
        let tags;

        {
            let loog = grih.loog.get(0).unwrap();
            
            if let None = loog.tags {
                return Resp::Err("You are not a randome vyakti!".to_owned());
            }

            addr = loog.addr.clone();
            name = loog.name.to_owned();
            tags = loog.tags.clone().unwrap();
        }        

        // Check if watch list is empty
        if self.vyaktigat_waitlist.len() == 0 {
            self.vyaktigat_waitlist.push(VyaktiWatchlist {
                kunjika: msg.kunjika,
                addr,
                name,
                tags
            });
            return Resp::None;
        }

        // connect person with tag or to zero
        let tags = tags.clone();
        let pos = match self.vyaktigat_waitlist.iter().position(|vk| {
            match vk.tags.iter().position(|t| tags.contains(t)) {
                Some(_) => true,
                None => false
            }
        }) {
            Some(i) => i,
            None => {
                self.vyaktigat_waitlist.push(VyaktiWatchlist {
                    kunjika: msg.kunjika,
                    addr,
                    name,
                    tags
                });
                return Resp::None;
            }
        };

        let vayakti_watchlist = self.vyaktigat_waitlist.remove(pos);
        let group_kunjika = format!("gupt_{}>{}",msg.kunjika.to_owned(), vayakti_watchlist.kunjika);
        grih.loog.remove(loog_i);
        grih.loog.iter().for_each(|a| {
            a.addr.do_send(ms::WsDisconnected {
                kunjika: msg.kunjika.to_owned(),
                name: name.to_owned()
            })
        });
        let log_count = grih.loog.len();
        drop(grih);
        if log_count == 0 {
            self.grih.remove(&msg.grih_kunjika);
        }
        self.grih.insert(group_kunjika.to_owned(), Grih {
            length: Some(2),
            loog: vec![Loog::new(addr.clone(), msg.kunjika.to_owned(), name.to_owned(), Some(tags.clone())),
                Loog::new(vayakti_watchlist.addr.clone(), vayakti_watchlist.kunjika.to_owned(), vayakti_watchlist.name.to_owned(), Some(vayakti_watchlist.tags.clone()))]
        });

        // notify about connection
        addr.do_send(ms::WsConnectedRandom {
            name: vayakti_watchlist.name,
            kunjika: vayakti_watchlist.kunjika,
            grih_kunjika: group_kunjika.to_owned()
        });

        vayakti_watchlist.addr.do_send(ms::WsConnectedRandom {
            name,
            kunjika: msg.kunjika.to_owned(),
            grih_kunjika: group_kunjika
        });

        Resp::Ok
    }
}

/// send text to everyone
impl Handler<ms::SendText> for ChatPinnd {
    type Result = ();

    fn handle(&mut self, msg: ms::SendText, _: &mut Self::Context) -> Self::Result {
        if let Some(grih) = self.grih.get(&msg.grih_kunjika) {
            grih.loog.iter().for_each(|c| {
                c.addr.do_send(ms::WsText {
                    sender_kunjika: msg.kunjika.to_owned(),
                    text: msg.text.to_owned(),
                    reply: msg.reply.to_owned()
                });
            });
        }
    }
}

/// send status to everyone
impl Handler<ms::SendStatus> for ChatPinnd {
    type Result = ();

    fn handle(&mut self, msg: ms::SendStatus, _: &mut Self::Context) -> Self::Result {
        if let Some(grih) = self.grih.get(&msg.grih_kunjika) {
            grih.loog.iter().for_each(|c| {
                if c.kunjika == msg.kunjika {
                    return;
                }
                c.addr.do_send(ms::WsStatus {
                    sender_kunjika: msg.kunjika.to_owned(),
                    status: msg.status.to_owned(),
                });
            });
        }
    }
}

/// send list of users
impl Handler<ms::List> for ChatPinnd {
    type Result = String;

    fn handle(&mut self, msg: ms::List, _: &mut Self::Context) -> Self::Result {
        if let Some(grih) = self.grih.get(&msg.grih_kunjika) {
            let mut list = Vec::new();
            for x in grih.loog.iter() {
                list.push((x.kunjika.to_owned(),x.name.to_owned()));
            }
            serde_json::json!(list).to_string()
        } else {
            "".to_string()
        }
    }
}

/// Notifiy a user disconnected and trim grih
impl Handler<ms::LeaveUser> for ChatPinnd {
    type Result = ();

    fn handle(&mut self, msg: ms::LeaveUser, _: &mut Self::Context) -> Self::Result {
        if let Some(grih_kunjika) = &msg.grih_kunjika {
            if let Some(grih) = self.grih.get_mut(grih_kunjika) {
                let name = if let Some(i) = grih.loog.iter().position(|x| x.addr == msg.addr) {
                    grih.loog.remove(i).name
                } else { "".to_owned() };
    
                if grih.loog.len() == 0 {
                    self.grih.remove(grih_kunjika);
                } else {
                    grih.loog.iter().for_each(|a| {
                        a.addr.do_send(ms::WsDisconnected {
                            kunjika: msg.kunjika.to_owned(),
                            name: name.to_owned()
                        })
                    });
                }
            }
        }
        
        if let Some(i) = self.vyaktigat_waitlist.iter().position(|a| a.kunjika == msg.kunjika) {
            self.vyaktigat_waitlist.remove(i);
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

impl Loog {
    fn new(addr: Addr<ws_sansad::WsSansad>,
        kunjika: String,
        name: String,
        tags: Option<Vec<String>>) -> Self {

        Loog {
            addr,
            kunjika,
            name,
            tags
        }
    }
}

impl SystemService for ChatPinnd {}
impl Supervised for ChatPinnd {}
