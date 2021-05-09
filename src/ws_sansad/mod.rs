//! Ws Sansad manage websocket of each client

mod handlers;
mod users;
mod messages;

use actix::prelude::*;
use actix_broker::{Broker, SystemBroker};
use actix_web_actors::ws;
use serde_json::{json, Value};
use std::time::{Duration, Instant};

use crate::{chat_pinnd::ChatPinnd, broker_messages as ms, broker_messages::util::Resp, validator::{Validation as vl, validate}};

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(15);

pub struct WsSansad {
    kunjika: String,
    isthiti: Isthiti,
    addr: Option<Addr<Self>>,
    hb: Instant
}

#[derive(Debug)]
enum Isthiti {
    None,
    Kaksh(String),
    VraktigatWaitlist
}


impl Actor for WsSansad {
    type Context = ws::WebsocketContext<Self>;
    
    fn started(&mut self, ctx: &mut Self::Context) {
        self.addr = Some(ctx.address().clone()); // own addr
        self.hb(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        futures::executor::block_on(self.leave_kaksh()); // notify leaving
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
            }, Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }, Ok(ws::Message::Text(msg)) => {
                futures::executor::block_on(self.parse_text_handle(msg));
            }, Ok(ws::Message::Close(msg)) => {
                ctx.close(msg);
                ctx.stop();
            }
            _ => ctx.stop()
        }
    }
}


impl WsSansad {
    pub fn new() -> Self {
        WsSansad {
            kunjika: String::new(),
            isthiti: Isthiti::None,
            addr: None,
            hb: Instant::now()
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
                futures::executor::block_on(act.leave_kaksh()); // notify leaving
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
            // let cmd  = match val.get("cmd") {
            //     Some(v) => v,
            //     None => return
            // };
            // let cmd  = match cmd.as_str() {
            //     Some(v) => v,
            //     None => return
            // };

            match val.get("cmd").unwrap().as_str().unwrap() {
                "join" => { self.join_kaksh(val).await },
                "rand" => { self.join_random(val).await },
                "randnext" => { self.join_random_next().await },
                "text" => { self.send_text(val).await },
                "status" => { self.send_status(val).await },
                "list" => { self.list().await },
                "leave" => { self.leave_kaksh().await },
                _ => ()
            }
        }
    }

    /// send ok response
    fn send_ok_response(&self, text: &str) {
        self.addr.clone().unwrap().do_send(ms::sansad::WsResponse {
            result: "Ok".to_owned(),
            message: text.to_owned()
        });
    }

    /// send error response
    fn send_err_response(&self, text: &str) {
        self.addr.clone().unwrap().do_send(ms::sansad::WsResponse {
            result: "Err".to_owned(),
            message: text.to_owned()
        });
    }
}