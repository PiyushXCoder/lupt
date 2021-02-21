//! Messages to be sent between Actors
use actix::prelude::*;
use dev::{MessageResponse, ResponseChannel};

use crate::ws_sansad::WsSansad;

//################################################## For ChatPinnd ##################################################
/// Request to change information of vayakti to list of vayakti im ChatPind

/// Request to Grih with its kunjika
#[derive(Clone, Message)]
#[rtype(result = "Resp")]
pub struct JoinGrih {
    pub grih_kunjika: String,
    pub length: Option<usize>,
    pub addr: Addr<WsSansad>,
    pub kunjika: String,
    pub name: String,
}

/// Request to connect Random vayakti
#[derive(Clone, Message)]
#[rtype(result = "Resp")]
pub struct JoinRandom {
    pub addr: Addr<WsSansad>,
    pub kunjika: String,
    pub name: String,
    pub tags: Vec<String>,
}
/// Request to connect Random vayakti
#[derive(Clone, Message)]
#[rtype(result = "Resp")]
pub struct JoinRandomNext {
    pub grih_kunjika: String,
    pub kunjika: String
}

/// Request to send text t
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct SendText {
    pub grih_kunjika: String,
    pub kunjika: String,
    pub text: String,
    pub reply: Option<String>,
}

// Request to send text t
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct SendStatus {
    pub grih_kunjika: String,
    pub kunjika: String,
    pub status: String
}

// Request to send text t
#[derive(Clone, Message)]
#[rtype(result = "String")]
pub struct List {
    pub grih_kunjika: String
}

/// Request to leave grih
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct  LeaveUser {
    pub grih_kunjika: Option<String>,
    pub kunjika: String,
    pub addr: Addr<WsSansad>
}

//################################################## For WsSansad ##################################################
// Request to send transfer text
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct WsText {
    pub text: String,
    pub reply: Option<String>,
    pub sender_kunjika: String
}

// Request to send transfer text
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct WsStatus {
    pub status: String,
    pub sender_kunjika: String
}

// Request to send transfer text
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct WsList {
    pub json: String
}

// Notify Someone connected
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct WsConnected {
    pub name: String,
    pub kunjika: String
}

// Notify someone disconnected
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct WsDisconnected {
    pub kunjika: String,
    pub name: String
}

// Give response message
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct WsResponse {
    pub result: String,
    pub message: String
}

// Got connected to random vayakti
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct WsConnectedRandom {
    pub name: String,
    pub kunjika: String,
    pub grih_kunjika: String
}
//################################################## Helper ##################################################
#[derive(Debug)]
pub enum Resp {
    Ok,
    Err(String), 
    None
}

impl<A, M> MessageResponse<A, M> for Resp
where
    A: Actor,
    M: Message<Result = Resp>,
{
    fn handle<R: ResponseChannel<M>>(self, _: &mut A::Context, tx: Option<R>) {
        if let Some(tx) = tx {
            tx.send(self);
        }
    }
}