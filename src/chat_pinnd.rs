//! Chat Pinnd(पिण्ड) is Actor to manage Websocket Chat related action

use std::{collections::HashMap, vec};

use actix::prelude::*;
use actix_broker::BrokerSubscribe;
use vecmap::VecMap;

use crate::{errors, ws_sansad, messages as ms};

#[allow(dead_code)]
pub struct ChatPinnd {
    grih: HashMap<String, Grih>, // kunjika, Grih
    vyaktigat_waitlist: Vec<VyaktiWatchlist>,
    non_connected_vyakti: VecMap<String, Vyakti>, // kunjika, vayakti
}

pub struct Grih {
    length: Option<usize>,
    loog: Vec<Loog>
}

pub struct Loog {
    addr: Addr<ws_sansad::WsSansad>,
    kunjika: String,
    name: String,
    _tags: Vec<String>
}

#[derive(Debug, Clone)]
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
        // for actix broker 
        self.subscribe_system_async::<ms::SendText>(ctx);
        self.subscribe_system_async::<ms::LeaveUser>(ctx);
    }
}

// Set information of user
impl Handler<ms::SetInfoVyakti> for ChatPinnd {
    type Result = Option<String>;

    fn handle(&mut self, msg: ms::SetInfoVyakti, _: &mut Self::Context) -> Self::Result {
        // check if vayakti info is not modified and do key exist
        if !msg.modify {
            if self.non_connected_vyakti.key_exist(&msg.kunjika) {
                return Some("Kunjika Exists".to_owned());
            } 
            if let Some(_) = self.grih.iter().position(|a| {
                match a.1.loog.iter().position(|b| {
                    b.kunjika == msg.kunjika
                }) {
                    Some(_) => true,
                    None => false
                }
            }) {
                return Some("Kunjika Exists".to_owned());
            }
        }
        // change value
        self.non_connected_vyakti.insert(msg.kunjika, Vyakti {
            name: msg.name,
            tags: msg.tags
        });

        None
    }
}

/// Join grih
impl Handler<ms::JoinGrih> for ChatPinnd {
    type Result = Result<(), errors::GrihFullError>;

    fn handle(&mut self, msg: ms::JoinGrih, _: &mut Self::Context) -> Self::Result {
        match self.grih.get_mut(&msg.grih_kunjika) { // check if group exist
            Some(grih) =>{ // exist
                // check if group have no space left
                if let Some(n) = grih.length {
                    if grih.loog.len() >= n {
                        return Err(errors::GrihFullError);
                    } 
                }

                let vayakti = self.non_connected_vyakti.get(&msg.kunjika).unwrap();
                let name = vayakti.name.to_owned();
                let tags = vayakti.tags.to_owned();

                let name_tmp = name.clone();
                let kunjika_tmp = msg.kunjika.clone();
                grih.loog.iter().for_each(move |a: &Loog| {
                    a.addr.do_send(ms::WsConnected {
                        name: name_tmp.clone(),
                        kunjika: kunjika_tmp.clone()
                    })
                });
                self.non_connected_vyakti.remove(&msg.kunjika).unwrap_or(());
                grih.loog.push(Loog::new(msg.addr, msg.kunjika,name,tags));

                
            }, None => { // don't exist
                // add group and notify
                let vayakti = self.non_connected_vyakti.get(&msg.kunjika).unwrap();
                msg.addr.do_send(ms::WsConnected {
                    name: vayakti.name.clone(),
                    kunjika: msg.kunjika.clone()
                });
                self.grih.insert(msg.grih_kunjika, Grih {
                    length: msg.length,
                    loog: vec![Loog::new(msg.addr,msg.kunjika.clone(),vayakti.name.clone(),vayakti.tags.clone())]
                });
                self.non_connected_vyakti.remove(&msg.kunjika).unwrap_or(());   
            }
        }

        Ok(())
    }
}

/// Join random vayakti 
/// Works as:
/// Check if watchlist is empty, if yes add the kunjika andaddr to watchlist
/// if watchlist have people get 0th  person an connect it
impl Handler<ms::JoinRandom> for ChatPinnd {
    type Result = Option<()>;
    fn handle(&mut self, msg: ms::JoinRandom, _: &mut Self::Context) -> Self::Result {
        // Check if watch list is empty
        if self.vyaktigat_waitlist.len() == 0 {
            self.vyaktigat_waitlist.push(VyaktiWatchlist {
                kunjika: msg.kunjika,
                addr: msg.addr
            });
            return None;
        }
        
        // connect 0th person
        let vayakti_watchlist = self.vyaktigat_waitlist.remove(0);
        let vayakti1_name: String;
        let vayakti2_name: String;
        let group_kunjika: String;
        {
            let vayakti1 =  self.non_connected_vyakti.get(&msg.kunjika).unwrap();
            let vayakti2 = self.non_connected_vyakti.get(&vayakti_watchlist.kunjika).unwrap();
            vayakti1_name = vayakti1.name.clone();
            vayakti2_name = vayakti2.name.clone();
            group_kunjika = format!("gupt_{}>{}",msg.kunjika.clone(), vayakti_watchlist.kunjika);
            self.grih.insert(group_kunjika.clone(), Grih {
                length: Some(2),
                loog: vec![Loog::new(msg.addr.clone(), msg.kunjika.clone(), vayakti1.name.clone(), vayakti1.tags.clone()),
                    Loog::new(vayakti_watchlist.addr.clone(), vayakti_watchlist.kunjika.clone(), vayakti2.name.clone(), vayakti2.tags.clone())]
            });
        }

        self.non_connected_vyakti.remove(&msg.kunjika).unwrap_or(());
        self.non_connected_vyakti.remove(&vayakti_watchlist.kunjika).unwrap_or(());

        // notify about connection
        msg.addr.do_send(ms::WsConnectedRandom {
            ajnyat_name: vayakti2_name,
            grih_kunjika: group_kunjika.clone()
        });

        vayakti_watchlist.addr.do_send(ms::WsConnectedRandom {
            ajnyat_name: vayakti1_name,
            grih_kunjika: group_kunjika
        });

        Some(())
    }
}

/// send text to everyone
impl Handler<ms::SendText> for ChatPinnd {
    type Result = ();

    fn handle(&mut self, msg: ms::SendText, _: &mut Self::Context) -> Self::Result {
        if let Some(grih) = self.grih.get(&msg.grih_kunjika) {
            grih.loog.iter().for_each(|c| {
                c.addr.do_send(ms::WsText {
                    sender_kunjika: msg.kunjika.clone(),
                    text: msg.text.clone(),
                    reply: msg.reply.clone()
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
                c.addr.do_send(ms::WsStatus {
                    sender_kunjika: msg.kunjika.clone(),
                    status: msg.status.clone(),
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
                list.push((x.kunjika.clone(),x.name.clone()));
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
                if let Some(i) = grih.loog.iter().position(|x| x.addr == msg.addr) {
                    grih.loog.remove(i);
                }
    
                if grih.loog.len() == 0 {
                    self.grih.remove(grih_kunjika);
                } else {
                    grih.loog.iter().for_each(|a| {
                        a.addr.do_send(ms::WsDisconnected {
                            kunjika: msg.kunjika.clone()
                        })
                    });
                }
            }
        }
        
        self.non_connected_vyakti.remove(&msg.kunjika).unwrap_or(());
        if let Some(i) = self.vyaktigat_waitlist.iter().position(|a| a.kunjika == msg.kunjika) {
            self.vyaktigat_waitlist.remove(i);
        }
    }
}

impl Default for ChatPinnd {
    fn default() -> Self {
        ChatPinnd {
            grih: HashMap::new(),
            vyaktigat_waitlist: Vec::new(),
            non_connected_vyakti: VecMap::new()
        }
    }
}

impl Loog {
    fn new(addr: Addr<ws_sansad::WsSansad>,
        kunjika: String,
        name: String,
        tags: Vec<String>) -> Self {
        Loog {
            addr,
            kunjika,
            name,
            _tags:tags
        }
    }
}

impl SystemService for ChatPinnd {}
impl Supervised for ChatPinnd {}
