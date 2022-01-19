/*
    This file is part of Lupt.

    Lupt is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    Lupt is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with Lupt.  If not, see <https://www.gnu.org/licenses/>
*/

use super::*;

//################################################## For WsSansad ##################################################
// Request to send own kunjika hash

// Notify Someone connected
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct WsConnected {
    pub name: String,
    pub kunjika: String,
}

// Got connected to random vayakti
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct WsConnectedRandom {
    pub name: String,
    pub kunjika: String,
    pub kaksh_kunjika: String,
}

// Request to send hash calculated of kunjika
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct WsKunjikaHash {
    pub kunjika: String,
}

// Request to send list
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct WsList {
    pub json: String,
}

// Notify someone disconnected
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct WsDisconnected {
    pub kunjika: String,
    pub name: String,
}

// Request to send Text
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct WsText {
    pub text: String,
    pub reply: Option<String>,
    pub sender_kunjika: String,
    pub msg_id: u128,
}

// Request to send Image
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct WsImage {
    pub src: String,
    pub sender_kunjika: String,
    pub msg_id: u128,
}
// Request to send Reaction
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct WsReaction {
    pub emoji: String,
    pub sender_kunjika: String,
    pub msg_id: String,
}

// Request to send Status
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct WsStatus {
    pub status: String,
    pub sender_kunjika: String,
}

// Request to delete messages
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct WsDeleteMsg {
    pub msg_id: Vec<String>,
    pub sender_kunjika: String,
}

// Request to edit messages
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct WsEditMsg {
    pub text: String,
    pub sender_kunjika: String,
    pub msg_id: String,
}

// Give response message
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct WsResponse {
    pub result: String,
    pub message: String,
}
