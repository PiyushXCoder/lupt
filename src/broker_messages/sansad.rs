/*
    This file is part of Tarangam.

    Tarangam is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    Tarangam is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with Tarangam.  If not, see <https://www.gnu.org/licenses/>
*/

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

// Request to send transfer Image
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct WsImage {
    pub src: String,
    pub sender_kunjika: String,
    pub msg_id: u128
}
// Request to send REaction
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct WsReaction {
    pub emoji: String,
    pub sender_kunjika: String,
    pub msg_id: String
}


// Request to send transfer status
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct WsStatus {
    pub status: String,
    pub sender_kunjika: String
}

// Request to delete messages
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct WsDeleteMsg {
    pub msg_id: Vec<String>,
    pub sender_kunjika: String
}

// Request to delete messages
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct WsEditMsg {
    pub text: String,
    pub sender_kunjika: String,
    pub msg_id: String
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
