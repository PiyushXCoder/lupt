
use super::*;


//################################################## For WsSansad ##################################################
// Request to send own kunjika hash
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct WsKunjikaHash {
    pub kunjika: String
}
// Request to send transfer text
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct WsText {
    pub text: String,
    pub reply: Option<String>,
    pub sender_kunjika: String,
    pub msg_id: u128
}

// Request to send transfer text
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct WsImage {
    pub src: String,
    pub sender_kunjika: String,
    pub msg_id: u128
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
    pub kaksh_kunjika: String
}
