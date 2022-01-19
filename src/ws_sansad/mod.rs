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

//! Ws Sansad manage websocket of each client

mod handlers;
mod messages;
mod users;

use actix::prelude::*;
use actix_broker::{Broker, SystemBroker};
use actix_web_actors::ws;
use serde_json::{json, Value};
use std::time::{Duration, Instant};

use crate::{
    broker_messages as ms,
    broker_messages::util::Resp,
    chat_pinnd::ChatPinnd,
    validator::{validate, Validation as vl},
};

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(15);

/// How often heartbeat pings are sent
const SPECIAL_HEARTBEAT_INTERVAL: Duration = Duration::from_secs(3 * 60);
/// How long before lack of client response causes a timeout
const SPECIAL_CLIENT_TIMEOUT: Duration = Duration::from_secs(15 * 60);

pub struct WsSansad {
    kunjika: String,
    isthiti: Isthiti,
    addr: Option<Addr<Self>>,
    hb: Instant,
    special_hb: Instant,
}

#[derive(Debug)]
enum Isthiti {
    None,
    Kaksh(String),
    VraktigatWaitlist,
}

impl Actor for WsSansad {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.addr = Some(ctx.address().clone()); // own addr
        self.hb(ctx);
        self.special_hb(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(self.leave_kaksh()); // notify leaving
        Running::Stop
    }
}

/// manage stream
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSansad {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                ctx.ping(&msg);
                self.hb = Instant::now();
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(msg)) => {
                self.special_hb = Instant::now();
                tokio::runtime::Runtime::new()
                    .unwrap()
                    .block_on(self.parse_text_handle(msg));
            }
            Ok(ws::Message::Close(msg)) => {
                ctx.close(msg);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

impl WsSansad {
    pub fn new() -> Self {
        WsSansad {
            kunjika: String::new(),
            isthiti: Isthiti::None,
            addr: None,
            hb: Instant::now(),
            special_hb: Instant::now(),
        }
    }

    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out

                // stop actor
                tokio::runtime::Runtime::new()
                    .unwrap()
                    .block_on(act.leave_kaksh());
                ctx.stop();
                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }

    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn special_hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(SPECIAL_HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.special_hb) > SPECIAL_CLIENT_TIMEOUT {
                // heartbeat timed out

                // stop actor
                tokio::runtime::Runtime::new()
                    .unwrap()
                    .block_on(act.leave_kaksh());
                ctx.stop();
                // don't try to send a ping
                return;
            }
            ctx.ping(b"");
        });
    }

    /// parse the request text from client
    async fn parse_text_handle(&mut self, msg: String) {
        if let Ok(val) = serde_json::from_str::<Value>(&msg) {
            match val.get("cmd").unwrap().as_str().unwrap() {
                "join" => self.join_kaksh(val).await,
                "rand" => self.join_random(val).await,
                "randnext" => self.join_random_next().await,
                "text" => self.send_text(val).await,
                "img" => self.send_image(val).await,
                "react" => self.send_reaction(val).await,
                "status" => self.send_status(val).await,
                "del" => self.delete_msg(val).await,
                "edit" => self.edit_msg(val).await,
                "list" => self.list().await,
                "leave" => self.leave_kaksh().await,
                _ => (),
            }
        }
    }

    /// send ok response
    fn send_ok_response(&self, text: &str) {
        self.addr.clone().unwrap().do_send(ms::sansad::WsResponse {
            result: "Ok".to_owned(),
            message: text.to_owned(),
        });
    }

    /// send error response
    fn send_err_response(&self, text: &str) {
        self.addr.clone().unwrap().do_send(ms::sansad::WsResponse {
            result: "Err".to_owned(),
            message: text.to_owned(),
        });
    }
}
