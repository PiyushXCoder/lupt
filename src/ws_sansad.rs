//! Ws Sansad manage websocket of each client
use actix::prelude::*;
use actix_broker::{Broker, SystemBroker};
use actix_web_actors::ws;
use serde_json::{json, Value};

use crate::chat_pinnd::ChatPinnd;
use crate::messages as ms;

pub struct WsSansad {
    name: String,
    isthiti: Isthiti,
    addr: Option<Addr<Self>>
}

#[allow(dead_code)]
enum Isthiti {
    None,
    Grih(Grih),
    // VraktigatWaitlist
}

pub struct Grih {
    kunjika: i32,
    // name: String
}

impl Actor for WsSansad {
    type Context = ws::WebsocketContext<Self>;
    
    fn started(&mut self, ctx: &mut Self::Context) {
        self.addr = Some(ctx.address().clone());
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        futures::executor::block_on(self.end());
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

impl WsSansad {
    pub fn new() -> Self {
        WsSansad {
            name: "()".to_owned(),
            isthiti: Isthiti::None,
            addr: None
        }
    }

    async fn parse_text_handle(&mut self, msg: String) {
        if let Ok(val) = serde_json::from_str::<Value>(&msg) {
            match val.get("cmd").unwrap().as_str().unwrap() {
                "name" => { self.name(val).await },
                "join" => { self.join(val).await },
                "text" => { self.text(val).await },
                "end" => { self.end().await },
                _ => ()
            }
        }
    }

    async fn name(&mut self, val: Value) {
        self.name = val.get("name").unwrap().as_str().unwrap().to_owned();
    }

    async fn text(&mut self, val: Value) {
        let text = val.get("text").unwrap().as_str().unwrap().to_owned();
        let grih_kunjika = match &self.isthiti {
            Isthiti::Grih(g) => {
                g.kunjika
            }, Isthiti::None => {
                return;
            }
        };

        Broker::<SystemBroker>::issue_async(ms::ReciveText {
            grih_kunjika,
            sender_name: self.name.clone(),
            text
        });
    }

    async fn join(&mut self, val: Value) {
        let name = val.get("name").unwrap().as_str().unwrap().to_owned();
        let length: Option<usize> = match val.get("length") {
            Some(val) => Some(val.as_i64().unwrap() as usize),
            None => None
        };

        let kunjika  = ChatPinnd::from_registry().send(ms::JoinUser{
            grih: ms::JoinUserGrihType::Name(name.clone()),
            length,
            addr: self.addr.clone().unwrap()
        }).await.unwrap().unwrap();

        self.isthiti = Isthiti::Grih(Grih {
            kunjika,
            // name
        })
    }

    async fn end(&mut self) {
        if let Isthiti::Grih(val) = &mut self.isthiti {
            Broker::<SystemBroker>::issue_async(ms::LeaveUser {
                grih_kunjika: val.kunjika,
                addr: self.addr.clone().unwrap()
            });
        }
    }
}
