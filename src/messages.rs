//! Messages to be sent between Actors
use actix::prelude::*;

use crate::ws_sansad::WsSansad;
use crate::errors;


//################################################## For ChatPinnd ##################################################
#[derive(Clone, Message)]
#[rtype(result = "Option<String>")] // None if no error
pub struct SetInfoVyakti {
    pub kunjika: String,
    pub name: String,
    pub tags: Vec<String>,
    pub modify: bool
}

#[derive(Clone, Message)]
#[rtype(result = "Result<(), errors::GrihFullError>")]
pub struct JoinGrih {
    pub grih_kunjika: String,
    pub length: Option<usize>,
    pub addr: Addr<WsSansad>,
    pub kunjika: String
}

#[derive(Clone, Message)]
#[rtype(result = "Option<()>")]
pub struct JoinRandom {
    pub addr: Addr<WsSansad>,
    pub kunjika: String
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct SendText {
    pub grih_kunjika: String,
    pub kunjika: String,
    pub text: String
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct  LeaveUser {
    pub grih_kunjika: String,
    pub kunjika: String,
    pub addr: Addr<WsSansad>
}

//################################################## For WsSansad ##################################################
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct WsMessage {
    pub text: String,
    pub sender: String
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct WsConnected {
    pub name: String,
    pub kunjika: String
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct WsDisconnected {
    pub kunjika: String
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct WsResponse {
    pub result: String,
    pub message: String
}


#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct WsConnectedRandom {
    pub ajnyat_name: String,
    pub grih_kunjika: String
}

