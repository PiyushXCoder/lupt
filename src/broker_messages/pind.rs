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

//! Messages to be sent between Actors
use super::responses::ResultResponse;
use super::*;

//################################################## For ChatPinnd ##################################################
/// Request to change information of vayakti to list of vayakti im ChatPind

/// Request to Kaksh with its kunjika
#[derive(Clone, Message)]
#[rtype(result = "ResultResponse")]
pub struct JoinKaksh {
    pub kaksh_kunjika: String,
    pub length: Option<usize>,
    pub addr: Addr<WsSansad>,
    pub kunjika: String,
    pub name: String,
}

/// Request to connect Random vayakti
#[derive(Clone, Message)]
#[rtype(result = "ResultResponse")]
pub struct JoinRandom {
    pub addr: Addr<WsSansad>,
    pub kunjika: String,
    pub name: String,
    pub tags: Vec<String>,
}
/// Request to connect Random Next vayakti
#[derive(Clone, Message)]
#[rtype(result = "ResultResponse")]
pub struct JoinRandomNext {
    pub kaksh_kunjika: String,
    pub kunjika: String,
}

// Request to send list of users
#[derive(Clone, Message)]
#[rtype(result = "String")]
pub struct List {
    pub kaksh_kunjika: String,
}

/// Request to leave kaksh
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct LeaveVayakti {
    pub kaksh_kunjika: Option<String>,
    pub kunjika: String,
    pub addr: Addr<WsSansad>,
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

/// Request to send image
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct SendImage {
    pub kaksh_kunjika: String,
    pub kunjika: String,
    pub src: String,
}
/// Request to reaction
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct SendReaction {
    pub kaksh_kunjika: String,
    pub kunjika: String,
    pub emoji: String,
    pub msg_id: String,
}

// Request to send status
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct SendStatus {
    pub kaksh_kunjika: String,
    pub kunjika: String,
    pub status: String,
}

// Request to delete messages
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct DeleteMsg {
    pub kaksh_kunjika: String,
    pub kunjika: String,
    pub msg_id: Vec<String>,
}

// Request to edit messages
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct EditMsg {
    pub kaksh_kunjika: String,
    pub kunjika: String,
    pub text: String,
    pub msg_id: String,
}
