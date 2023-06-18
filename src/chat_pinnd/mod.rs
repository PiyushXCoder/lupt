/*
    This file is part of Lupt.

    Lupt is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    Lupt is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with Lupt.  If not, see <https://www.gnu.org/licenses/>
*/

//! Chat Pinnd(पिण्ड) is Actor to manage Websocket Chat related action

mod message;
mod user;

use std::{collections::HashMap, vec};

use actix::prelude::*;
use actix_broker::BrokerSubscribe;

use crate::{broker_messages as ms, broker_messages::responses::ResultResponse, ws_sansad};

#[allow(dead_code)]
pub struct ChatPinnd {
    kaksh: HashMap<String, Kaksh>, // kunjika, Kaksh
    vyaktigat_waitlist: Vec<VyaktiWatchlist>,
}

pub struct Kaksh {
    length: Option<usize>,
    last_message_id: u128,
    loog: Vec<Loog>,
}

pub struct Loog {
    addr: Addr<ws_sansad::WsSansad>,
    kunjika: String,
    name: String,
    tags: Option<Vec<String>>,
}

// #[derive(Debug, Clone)]
// pub struct Vyakti {
//     name: String,
//     tags: Vec<String>
// }
pub struct VyaktiWatchlist {
    kunjika: String,
    name: String,
    tags: Vec<String>,
    addr: Addr<ws_sansad::WsSansad>,
}

impl Actor for ChatPinnd {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // for actix broker
        self.subscribe_system_async::<ms::pind::SendText>(ctx);
        self.subscribe_system_async::<ms::pind::SendImage>(ctx);
        self.subscribe_system_async::<ms::pind::SendReaction>(ctx);
        self.subscribe_system_async::<ms::pind::DeleteMsg>(ctx);
        self.subscribe_system_async::<ms::pind::EditMsg>(ctx);
        self.subscribe_system_async::<ms::pind::SendStatus>(ctx);
        self.subscribe_system_async::<ms::pind::LeaveVayakti>(ctx);
    }
}

impl Default for ChatPinnd {
    fn default() -> Self {
        ChatPinnd {
            kaksh: HashMap::new(),
            vyaktigat_waitlist: Vec::new(),
        }
    }
}

impl Loog {
    fn new(
        addr: Addr<ws_sansad::WsSansad>,
        kunjika: String,
        name: String,
        tags: Option<Vec<String>>,
    ) -> Self {
        Loog {
            addr,
            kunjika,
            name,
            tags,
        }
    }
}

impl SystemService for ChatPinnd {}
impl Supervised for ChatPinnd {}
