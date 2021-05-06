//! Ws Sansad manage websocket of each client
use actix::prelude::*;
use actix_broker::{Broker, SystemBroker};
use actix_web_actors::ws;
use ms::Resp;
use serde_json::{json, Value};
use std::time::{Duration, Instant};

use crate::{chat_pinnd::ChatPinnd, messages as ms, validator::{Validation as vl, validate}};

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(15);

// for phones if browser kept websocket on 
const SPECIAL_HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5*60);
const SPECIAL_CLIENT_TIMEOUT: Duration = Duration::from_secs(10*60);

pub struct WsSansad {
    kunjika: String,
    isthiti: Isthiti,
    addr: Option<Addr<Self>>,
    hb: Instant,
    special_hb: Instant
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
        self.special_hb(ctx);
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
                self.special_hb = Instant::now();
                futures::executor::block_on(self.parse_text_handle(msg));
            }, Ok(ws::Message::Close(msg)) => {
                ctx.close(msg);
                ctx.stop();
            }
            _ => ctx.stop()
        }
    }
}

/// send text message
impl Handler<ms::WsText> for WsSansad {
    type Result = ();
    fn handle(&mut self, msg: ms::WsText, ctx: &mut Self::Context) -> Self::Result {
        let json = json!({
            "cmd": "text",
            "text": msg.text,
            "reply": msg.reply,
            "kunjika": msg.sender_kunjika // Sender's kunjuka
        });
        ctx.text(json.to_string());
    }
}


/// send text status
impl Handler<ms::WsStatus> for WsSansad {
    type Result = ();
    fn handle(&mut self, msg: ms::WsStatus, ctx: &mut Self::Context) -> Self::Result {
        let json = json!({
            "cmd": "status",
            "status": msg.status,
            "kunjika": msg.sender_kunjika // Sender's kunjuka
        });
        ctx.text(json.to_string());
    }
}

/// List Vayakti
impl Handler<ms::WsList> for WsSansad {
    type Result = ();
    fn handle(&mut self, msg: ms::WsList, ctx: &mut Self::Context) -> Self::Result {
        let json = json!({
            "cmd": "list",
            "vayakti": msg.json
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
            "name": msg.name,
            "kunjika": msg.kunjika
        });
        ctx.text(json.to_string());
    }
}

/// notify got connected to random person
impl Handler<ms::WsConnectedRandom> for WsSansad {
    type Result = ();
    fn handle(&mut self, msg: ms::WsConnectedRandom, ctx: &mut Self::Context) -> Self::Result {
        self.isthiti = Isthiti::Kaksh(msg.kaksh_kunjika);
        let json = json!({
            "cmd": "random",
            "name": msg.name,
            "kunjika": msg.kunjika
        });
        ctx.text(json.to_string());
    }
}

impl WsSansad {
    pub fn new() -> Self {
        WsSansad {
            kunjika: String::new(),
            isthiti: Isthiti::None,
            addr: None,
            hb: Instant::now(),
            special_hb: Instant::now()
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

    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn special_hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(SPECIAL_HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.special_hb) > SPECIAL_CLIENT_TIMEOUT {
                // heartbeat timed out

                // stop actor
                futures::executor::block_on(act.leave_kaksh()); // notify leaving
                ctx.stop();

                // don't try to send a ping
                return;
            }
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
        self.addr.clone().unwrap().do_send(ms::WsResponse {
            result: "Ok".to_owned(),
            message: text.to_owned()
        });
    }

    /// send error response
    fn send_err_response(&self, text: &str) {
        self.addr.clone().unwrap().do_send(ms::WsResponse {
            result: "Err".to_owned(),
            message: text.to_owned()
        });
    }
    /// Request for joining to random person
    async fn join_random(&mut self, val: Value) {
        // Check is already joined
        match self.isthiti {
            Isthiti::None => (),
            Isthiti::VraktigatWaitlist => {
                self.send_ok_response("watchlist");
                return;
            }, Isthiti::Kaksh(_) => return
        }

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
                Vec::new()
            }
        };

        // Validate
        if let Some(val ) = validate(vec![vl::NonEmpty, vl::NoSpace, vl::NoHashtag], &kunjika, "Kunjika") {
            self.send_err_response(&val);
            return;
        } else if let Some(val ) = validate(vec![vl::NonEmpty], &name, "Name") {
            self.send_err_response(&val);
            return;
        }

        // request
        let result: Resp = ChatPinnd::from_registry().send(ms::JoinRandom{
            addr: self.addr.clone().unwrap(),
            kunjika: kunjika.to_owned(),
            name,
            tags
        }).await.unwrap();

        match result {
            Resp::Err(err) => self.send_err_response(&err), 
            Resp::Ok =>  self.kunjika = kunjika,
            Resp::None => {
                self.addr.clone().unwrap().do_send(ms::WsResponse{
                    result: "watch".to_owned() ,
                    message: "Watchlist".to_owned()
                 });
                self.isthiti = Isthiti::VraktigatWaitlist;
                self.kunjika = kunjika
            }
        }
    }

    /// Request for joining to random person
    async fn join_random_next(&mut self) {
        // Check is already joined
        let kaksh_kunjika = match &self.isthiti {
            Isthiti::VraktigatWaitlist => {
                self.send_ok_response("watchlist");
                return;
            },
            Isthiti::Kaksh(kaksh_kunjika) => kaksh_kunjika,
            Isthiti::None => {
                self.send_ok_response("Not allowed");
                return;
            }
        };

        // request
        let result: Resp = ChatPinnd::from_registry().send(ms::JoinRandomNext {
            kunjika: self.kunjika.to_owned(),
            kaksh_kunjika: kaksh_kunjika.to_owned(),
        }).await.unwrap();

        match result {
            Resp::Err(err) => self.send_err_response(&err), 
            Resp::None => {
                self.addr.clone().unwrap().do_send(ms::WsResponse{
                   result: "watch".to_owned() ,
                   message: "Watchlist".to_owned()
                });
                self.isthiti = Isthiti::VraktigatWaitlist;
                self.kunjika = self.kunjika.to_owned()
            }
            _ => ()
        }
    }

    /// Request to join to kaksh
    async fn join_kaksh(&mut self, val: Value) {
        // Check is already joined
        match self.isthiti {
            Isthiti::None => (),
            _ => return
        }

        // is vayakti in watch list
        if let Isthiti::VraktigatWaitlist = self.isthiti {
            self.send_ok_response("watchlist");
            return;
        }

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
        let kaksh_kunjika = match val.get("kaksh_kunjika") {
            Some(val ) => val.as_str().unwrap().to_owned(),
            None => {
                self.send_err_response("Invalid request");
                return;
            }
        };
        let length: Option<usize> = match val.get("length") {
            Some(val) => match val.as_i64(){
                    Some(val) => Some(val as usize),
                    None => None
                },
            None => None
        };


        // Validate
        if let Some(val ) = validate(vec![vl::NonEmpty, vl::NoGupt, vl::NoSpace], &kaksh_kunjika, "Kaksh Kunjika") {
            self.send_err_response(&val);
            return;
        } else if let Some(val ) = validate(vec![vl::NonEmpty, vl::NoSpace, vl::NoHashtag], &kunjika, "Kunjika") {
            self.send_err_response(&val);
            return;
        } else if let Some(val ) = validate(vec![vl::NonEmpty], &name, "Name") {
            self.send_err_response(&val);
            return;
        }
        
        // request
        let result: Resp = ChatPinnd::from_registry().send(ms::JoinKaksh {
            kaksh_kunjika: kaksh_kunjika.to_owned(),
            length,
            addr: self.addr.clone().unwrap(),
            kunjika: kunjika.to_owned(),
            name
        }).await.unwrap();


        match result {
            Resp::Err(err) => self.send_err_response(&err), 
            Resp::Ok => {
                self.isthiti = Isthiti::Kaksh(kaksh_kunjika);
                self.kunjika = kunjika;
                self.send_ok_response("joined")
            }
            _ => ()
        }
    }

    /// Request to join to kaksh
    async fn list(&mut self) {
        // check if vayakti exist
        if let Isthiti::None = self.isthiti {
            self.send_err_response("Not in any Kaksh");
            return;
        }

        // check if connected to any kaksh
        match &self.isthiti {
            Isthiti::Kaksh(kunjika) => {
                let json: String = ChatPinnd::from_registry().send(ms::List {
                    kaksh_kunjika: kunjika.to_owned()
                }).await.unwrap();

                self.addr.clone().unwrap().do_send(ms::WsList {
                    json
                })
            },
            _ => {
                self.send_err_response("Kaksh not connected");
                return;
            }
        }
    }

    /// send text to vayakti in kaksh
    async fn send_text(&mut self, val: Value) {
        // check if vayakti exist
        if let Isthiti::None = self.isthiti {
            self.send_err_response("Not in any Kaksh");
            return;
        }

        // check if connected to any kaksh
        match self.isthiti {
            Isthiti::Kaksh(_) => (),
            _ => {
                self.send_err_response("Kaksh not connected");
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

        let reply: Option<String> = match val.get("reply") {
            Some(val) => Some(val.as_str().unwrap().to_owned()),
            None => None
        };

        let kaksh_kunjika = match &self.isthiti {
            Isthiti::Kaksh(kaksh_kunjika) => {
                kaksh_kunjika.to_owned()
            }, _ => {
                return;
            }
        };
        Broker::<SystemBroker>::issue_async(ms::SendText {
            kaksh_kunjika,
            kunjika: self.kunjika.to_owned(),
            text,
            reply
        });
    }

    /// send status to vayakti in kaksh
    async fn send_status(&mut self, val: Value) {
        // check if vayakti exist
        if let Isthiti::None = self.isthiti {
            self.send_err_response("Not in any Kaksh");
            return;
        }

        // check if connected to any kaksh
        match self.isthiti {
            Isthiti::Kaksh(_) => (),
            _ => {
                self.send_err_response("Kaksh not connected");
                return;
            }
        }

        // sent status
        let status = match val.get("status") {
            Some(val) => val,
            None => {
                self.send_err_response("Invalid request");
                return;
            }
        }.as_str().unwrap().to_owned();
        let kaksh_kunjika = match &self.isthiti {
            Isthiti::Kaksh(kaksh_kunjika) => {
                kaksh_kunjika.to_owned()
            }, _ => {
                return;
            }
        };
        Broker::<SystemBroker>::issue_async(ms::SendStatus {
            kaksh_kunjika,
            kunjika: self.kunjika.to_owned(),
            status
        });
    }

    /// notify leaving
    async fn leave_kaksh(&mut self) {
        let kaksh_kunjika = match &self.isthiti {
            Isthiti::Kaksh(val) => Some(val.to_owned()),
            _ => None
        };
        
        Broker::<SystemBroker>::issue_async(ms::LeaveUser {
            kaksh_kunjika,
            kunjika: self.kunjika.to_owned(),
            addr: self.addr.clone().unwrap()
        });
    
        self.isthiti = Isthiti::None;        
        self.send_ok_response("left");
    }
}
