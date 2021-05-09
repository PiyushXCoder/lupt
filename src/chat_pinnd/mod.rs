//! Chat Pinnd(पिण्ड) is Actor to manage Websocket Chat related action

mod user;
mod message; 

use std::{collections::HashMap, vec};

use actix::prelude::*;
use actix_broker::BrokerSubscribe;

use crate::{ws_sansad, broker_messages as ms, broker_messages::util::Resp};

#[allow(dead_code)]
pub struct ChatPinnd {
    kaksh: HashMap<String, Kaksh>, // kunjika, Kaksh
    vyaktigat_waitlist: Vec<VyaktiWatchlist>,
}

pub struct Kaksh {
    length: Option<usize>,
    last_message_id: u128,
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
        self.subscribe_system_async::<ms::pind::SendText>(ctx);
        self.subscribe_system_async::<ms::pind::SendStatus>(ctx);
        self.subscribe_system_async::<ms::pind::LeaveUser>(ctx);
    }
}


impl Default for ChatPinnd {
    fn default() -> Self {
        ChatPinnd {
            kaksh: HashMap::new(),
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

