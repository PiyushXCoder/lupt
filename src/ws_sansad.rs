//! Ws Sansad manage websocket of each client
use actix::prelude::*;
use actix_broker::{Broker, SystemBroker};
use actix_web_actors::ws;
use serde_json::{json, Value};

use crate::{chat_pinnd::ChatPinnd, messages as ms};
use crate::errors;

pub struct WsSansad {
    name: String,
    kunjika: Option<String>,
    isthiti: Isthiti,
    addr: Option<Addr<Self>>,
}

#[allow(dead_code)]
enum Isthiti {
    None,
    Grih(String),
    // VraktigatWaitlist
}

impl Actor for WsSansad {
    type Context = ws::WebsocketContext<Self>;
    
    fn started(&mut self, ctx: &mut Self::Context) {
        self.addr = Some(ctx.address().clone());
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        futures::executor::block_on(self.leave_grih());
        Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSansad {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.ping(&msg),
            Ok(ws::Message::Text(msg)) => {
                futures::executor::block_on(self.parse_text_handle(msg));
            },
            Ok(ws::Message::Close(msg)) => {
                ctx.close(msg);
            }
            _ => ctx.close(None)
        }
    }
}

impl Handler<ms::WsMessage> for WsSansad {
    type Result = ();
    fn handle(&mut self, msg: ms::WsMessage, ctx: &mut Self::Context) -> Self::Result {
        let json = json!({
            "cmd": "text",
            "text": msg.text,
            "sender": msg.sender
        });
        ctx.text(json.to_string());
    }
}

impl Handler<ms::WsResponse> for WsSansad {
    type Result = ();
    fn handle(&mut self, msg: ms::WsResponse, ctx: &mut Self::Context) -> Self::Result {
        let json = json!({
            "cmd": "resp",
            "result": msg.result,
            "message": msg.message
        });
        ctx.text(json.to_string());
    }
}

impl Handler<ms::WsConnected> for WsSansad {
    type Result = ();
    fn handle(&mut self, msg: ms::WsConnected, ctx: &mut Self::Context) -> Self::Result {
        let json = json!({
            "cmd": "connected",
            "user": msg.user
        });
        ctx.text(json.to_string());
    }
}

impl Handler<ms::WsDisconnected> for WsSansad {
    type Result = ();
    fn handle(&mut self, msg: ms::WsDisconnected, ctx: &mut Self::Context) -> Self::Result {
        let json = json!({
            "cmd": "disconnected",
            "user": msg.user
        });
        ctx.text(json.to_string());
    }
}


impl WsSansad {
    pub fn new() -> Self {
        WsSansad {
            name: "()".to_owned(),
            kunjika: None,
            isthiti: Isthiti::None,
            addr: None,
        }
    }

    async fn parse_text_handle(&mut self, msg: String) {
        println!("{}", msg);
        if let Ok(val) = serde_json::from_str::<Value>(&msg) {
            match val.get("cmd").unwrap().as_str().unwrap() {
                "name" => { self.set_name(val).await },
                "kunjika" => { self.set_kunjika(val).await },
                "join" => { self.join_grih(val).await },
                "text" => { self.send_text(val).await },
                "leave" => { self.leave_grih().await },
                _ => ()
            }
        }
    }

    fn send_ok_response(&self) {
        self.addr.clone().unwrap().do_send(ms::WsResponse {
            result: "Ok".to_owned(),
            message: "".to_owned()
        });
    }

    fn send_err_response(&self, text: &str) {
        self.addr.clone().unwrap().do_send(ms::WsResponse {
            result: "Err".to_owned(),
            message: text.to_owned()
        });
    }

    async fn set_name(&mut self, val: Value) {
        self.name = val.get("name").unwrap().as_str().unwrap().to_owned();
        self.send_ok_response();
    }

    async fn set_kunjika(&mut self, val: Value) {
        self.name = val.get("name").unwrap().as_str().unwrap().to_owned();
        self.send_ok_response();
    }

    async fn join_grih(&mut self, val: Value) {
        if let None = self.kunjika {
            self.send_err_response("No user kunjika set");
            return;
        }
        
        let kunjika = match val.get("kunjika") {
            Some(val) => val,
            None => {
                self.send_err_response("Invalid request");
                return;
            }
        }.as_str().unwrap().to_owned();
        let length: Option<usize> = match val.get("length") {
            Some(val) => Some(val.as_i64().unwrap() as usize),
            None => None
        };

        let result: Result<(), errors::GrihFullError> = ChatPinnd::from_registry().send(ms::JoinUser {
            grih_kunjika: kunjika.clone(),
            length,
            addr: self.addr.clone().unwrap(),
            name: self.name.clone()
        }).await.unwrap();

        match result {
            Ok(_) => {
                self.isthiti = Isthiti::Grih(kunjika);
                self.send_ok_response()
            },
            Err(e) => self.send_err_response(&format!("{}", e))
        }
    }

    async fn send_text(&mut self, val: Value) {
        let text = match val.get("text") {
            Some(val) => val,
            None => {
                self.send_err_response("Invalid request");
                return;
            }
        }.as_str().unwrap().to_owned();
        let grih_kunjika = match &self.isthiti {
            Isthiti::Grih(g) => {
                g.clone()
            }, Isthiti::None => {
                return;
            }
        };
        Broker::<SystemBroker>::issue_async(ms::SendText {
            grih_kunjika,
            sender_name: self.name.clone(),
            text
        });
    }

    async fn leave_grih(&mut self) {
        if let Isthiti::Grih(val) = &mut self.isthiti {
            Broker::<SystemBroker>::issue_async(ms::LeaveUser {
                grih_kunjika: val.clone(),
                addr: self.addr.clone().unwrap()
            });

            self.send_ok_response();
        }
    }
}
