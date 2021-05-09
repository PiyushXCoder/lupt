//! Messages to be sent between Actors
use super::*;
use super::util::Resp;

//################################################## For ChatPinnd ##################################################
/// Request to change information of vayakti to list of vayakti im ChatPind

/// Request to Kaksh with its kunjika
#[derive(Clone, Message)]
#[rtype(result = "Resp")]
pub struct JoinKaksh {
    pub kaksh_kunjika: String,
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
    pub kaksh_kunjika: String,
    pub kunjika: String
}

/// Request to send text 
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct SendText {
    pub kaksh_kunjika: String,
    pub kunjika: String,
    pub text: String,
    pub reply: Option<String>,
}
/// Request to send image t
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct SendImage {
    pub kaksh_kunjika: String,
    pub kunjika: String,
    pub part: String,
    pub image_id: i32
}

// Request to send text t
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct SendStatus {
    pub kaksh_kunjika: String,
    pub kunjika: String,
    pub status: String
}

// Request to send text t
#[derive(Clone, Message)]
#[rtype(result = "String")]
pub struct List {
    pub kaksh_kunjika: String
}

/// Request to leave kaksh
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct  LeaveUser {
    pub kaksh_kunjika: Option<String>,
    pub kunjika: String,
    pub addr: Addr<WsSansad>
}