//! Ws Sansad manage websocket of each client
use actix::prelude::*;
use actix_broker::{Broker, SystemBroker};
use actix_web_actors::ws;
use serde_json::{json, Value};

use crate::{chat_pinnd::ChatPinnd, messages as ms};
use crate::errors;

pub struct WsSansad {
    kunjika: Option<String>,
    isthiti: Isthiti,
    addr: Option<Addr<Self>>,
}

#[derive(Debug)]
enum Isthiti {
    None,
    Grih(String),
    VraktigatWaitlist
}

impl Actor for WsSansad {
    type Context = ws::WebsocketContext<Self>;
    
    fn started(&mut self, ctx: &mut Self::Context) {
        self.addr = Some(ctx.address().clone()); // own addr
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        futures::executor::block_on(self.leave_grih()); // notify leaving
        Running::Stop
    }
}

/// manage stream
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

/// send text message
impl Handler<ms::WsMessage> for WsSansad {
    type Result = ();
    fn handle(&mut self, msg: ms::WsMessage, ctx: &mut Self::Context) -> Self::Result {
        let json = json!({
            "cmd": "text",
            "text": msg.text,
            "kunjika": msg.sender // Sender's kunjuka
        });
        ctx.text(json.to_string());
    }
}

/// send response ok, error
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

/// notify someone got connected
impl Handler<ms::WsConnected> for WsSansad {
    type Result = ();
    fn handle(&mut self, msg: ms::WsConnected, ctx: &mut Self::Context) -> Self::Result {
        let json = json!({
            "cmd": "connected",
            "name": msg.name,
            "kunjika": msg.kunjika
        });
        ctx.text(json.to_string());
    }
}

/// notify someone got disconnected
impl Handler<ms::WsDisconnected> for WsSansad {
    type Result = ();
    fn handle(&mut self, msg: ms::WsDisconnected, ctx: &mut Self::Context) -> Self::Result {
        let json = json!({
            "cmd": "disconnected",
            "name": msg.kunjika
        });
        ctx.text(json.to_string());
    }
}

/// notify got connected to random person
impl Handler<ms::WsConnectedRandom> for WsSansad {
    type Result = ();
    fn handle(&mut self, msg: ms::WsConnectedRandom, ctx: &mut Self::Context) -> Self::Result {
        self.isthiti = Isthiti::Grih(msg.grih_kunjika);
        let json = json!({
            "cmd": "connected",
            "ajnyat": msg.ajnyat_name
        });
        ctx.text(json.to_string());
    }
}

impl WsSansad {
    pub fn new() -> Self {
        WsSansad {
            kunjika: None,
            isthiti: Isthiti::None,
            addr: None,
        }
    }

    /// parse the request text from client
    async fn parse_text_handle(&mut self, msg: String) {
        if let Ok(val) = serde_json::from_str::<Value>(&msg) {
            match val.get("cmd").unwrap().as_str().unwrap() {
                "seinfo" => { self.set_info(val).await },
                "join" => { self.join_grih(val).await },
                "rand" => { self.join_random().await },
                "text" => { self.send_text(val).await },
                "leave" => { self.leave_grih().await },
                _ => ()
            }
        }
    }

    /// send ok response
    fn send_ok_response(&self) {
        self.addr.clone().unwrap().do_send(ms::WsResponse {
            result: "Ok".to_owned(),
            message: "".to_owned()
        });
    }

    /// send error response
    fn send_err_response(&self, text: &str) {
        self.addr.clone().unwrap().do_send(ms::WsResponse {
            result: "Err".to_owned(),
            message: text.to_owned()
        });
    }

    /// send info of user and modify if needed
    async fn set_info(&mut self, val: Value) {
        // parse parameters
        let kunjika  = match val.get("kunjika") {
            Some(val ) => val.as_str().unwrap().to_owned(),
            None => {
                self.send_err_response("Invalid request");
                return;
            }
        };
        let name  = match val.get("name") {
            Some(val ) => val.as_str().unwrap().to_owned(),
            None => {
                self.send_err_response("Invalid request");
                return;
            }
        };
        let tags  = match val.get("tags") {
            Some(val ) => {
                let mut v = Vec::new();
                for x in val.as_str().unwrap().split_ascii_whitespace() {
                    v.push(x.to_owned());
                }
                v
            },
            None => {
                self.send_err_response("Invalid request");
                return;
            }
        };

        // check if eing modified
        let modify = self.kunjika == Some(kunjika.clone());

        //request
        let result: Option<String> = ChatPinnd::from_registry().send(ms::SetInfoVyakti {
            kunjika: kunjika.clone(),
            name,
            tags,
            modify
        }).await.unwrap();

        if let Some(msg) = result {
            self.send_err_response(&msg);
            return;
        }

        self.kunjika = Some(kunjika);
        self.send_ok_response();
    }

    /// Request for joining to random person
    async fn join_random(&mut self) {
        // check if vayakti exist
        if let None = self.kunjika {
            self.send_err_response("No vayakti kunjika set");
            return;
        }

        // is vayakti in watch list
        if let Isthiti::VraktigatWaitlist = self.isthiti {
            self.addr.clone().unwrap().do_send(ms::WsResponse {
                result: "Ok".to_owned(),
                message: "watchlist".to_owned()
            });
            return;
        }

        // request
        let result: Option<()> = ChatPinnd::from_registry().send(ms::JoinRandom{
            addr: self.addr.clone().unwrap(),
            kunjika: self.kunjika.clone().unwrap()
        }).await.unwrap();

        if let None = result {
            self.addr.clone().unwrap().do_send(ms::WsResponse {
                result: "Ok".to_owned(),
                message: "watchlist".to_owned()
            });
            self.isthiti = Isthiti::VraktigatWaitlist;
        }
    }

    /// Request to join to grih
    async fn join_grih(&mut self, val: Value) {
        //echeck user exist
        if let None = self.kunjika {
            self.send_err_response("No vayakti kunjika set");
            return;
        }
        
        // parse parameter
        let grih_kunjika = match val.get("kunjika") {
            Some(val) => val,
            None => {
                self.send_err_response("Invalid request");
                return;
            }
        }.as_str().unwrap().to_owned();

        // restrict place for vaktigat chat
        if grih_kunjika.starts_with("gupt_") {
            self.send_err_response("Such grih kunjika is restricted");
            return;
        }

        let length: Option<usize> = match val.get("length") {
            Some(val) => Some(val.as_i64().unwrap() as usize),
            None => None
        };

        // requesy
        let result: Result<(), errors::GrihFullError> = ChatPinnd::from_registry().send(ms::JoinGrih {
            grih_kunjika: grih_kunjika.clone(),
            length,
            addr: self.addr.clone().unwrap(),
            kunjika: self.kunjika.clone().unwrap()
        }).await.unwrap();

        match result {
            Ok(_) => {
                self.isthiti = Isthiti::Grih(grih_kunjika);
                self.send_ok_response()
            },
            Err(e) => self.send_err_response(&format!("{}", e))
        }
    }

    /// send text to vayakti in grih
    async fn send_text(&mut self, val: Value) {
        // check if vayakti exist
        if let None = self.kunjika {
            self.send_err_response("No vayakti kunjika set");
            return;
        }

        // check if connected to any grih
        match self.isthiti {
            Isthiti::Grih(_) => (),
            _ => {
                self.send_err_response("Grih not connected");
                return;
            }
        }

        // sent text
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
            }, _ => {
                return;
            }
        };
        Broker::<SystemBroker>::issue_async(ms::SendText {
            grih_kunjika,
            kunjika: self.kunjika.clone().unwrap(),
            text
        });
    }

    // notify leaving
    async fn leave_grih(&mut self) {
        if let Isthiti::Grih(val) = &mut self.isthiti {
            Broker::<SystemBroker>::issue_async(ms::LeaveUser {
                grih_kunjika: val.clone(),
                kunjika: self.kunjika.clone().unwrap(),
                addr: self.addr.clone().unwrap()
            });

            self.send_ok_response();
        }
    }
}
