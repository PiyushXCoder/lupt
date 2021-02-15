//! Messages to be sent between Actors
use actix::prelude::*;

use crate::ws_sansad::WsSansad;
use crate::errors;

// For ChatPinnd 
#[derive(Clone, Message)]
#[rtype(result = "Result<(), errors::AlreadyExistError>")]
pub struct SetKunjikaUser {
    pub kunjika: String
}

#[derive(Clone, Message)]
#[rtype(result = "Result<(), errors::GrihFullError>")]
pub struct JoinUser {
    pub grih_kunjika: String,
    pub length: Option<usize>,
    pub addr: Addr<WsSansad>,
    pub name: String
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct SendText {
    pub grih_kunjika: String,
    pub sender_name: String,
    pub text: String
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct  LeaveUser {
    pub grih_kunjika: String,
    pub addr: Addr<WsSansad>
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct  AddUserKunjika {
    pub old_kunjika: Option<String>,
    pub kunjika: String
}

// For WsSansad
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct WsMessage {
    pub text: String,
    pub sender: String
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct WsConnected {
    pub user: String
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct WsDisconnected {
    pub user: String
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct WsResponse {
    pub result: String,
    pub message: String
}
