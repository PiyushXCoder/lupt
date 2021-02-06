//! Messages to be sent between Actors
use actix::prelude::*;

use crate::ws_sansad::WsSansad;

// For ChatPinnd 
#[derive(Clone, Message)]
#[rtype(result = "Option<i32>")]
pub struct JoinUser {
    pub grih: JoinUserGrihType,
    pub length: Option<usize>,
    pub addr: Addr<WsSansad>
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum JoinUserGrihType {
    Name(String),
    Kunjika(i32)
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct ReciveText {
    pub grih_kunjika: i32,
    pub sender_name: String,
    pub text: String
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct  LeaveUser {
    pub grih_kunjika: i32,
    pub addr: Addr<WsSansad>
}

// For WsSansad
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct WsMessage {
    pub text: String,
    pub sender: String
} 