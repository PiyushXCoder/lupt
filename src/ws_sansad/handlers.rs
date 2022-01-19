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

/// notify someone got connected
impl Handler<ms::sansad::WsConnected> for WsSansad {
    type Result = ();
    fn handle(&mut self, msg: ms::sansad::WsConnected, ctx: &mut Self::Context) -> Self::Result {
        let json = json!({
            "cmd": "connected",
            "name": msg.name,
            "kunjika": msg.kunjika
        });
        ctx.text(json.to_string());
    }
}

/// notify got connected as random person
impl Handler<ms::sansad::WsConnectedRandom> for WsSansad {
    type Result = ();
    fn handle(
        &mut self,
        msg: ms::sansad::WsConnectedRandom,
        ctx: &mut Self::Context,
    ) -> Self::Result {
        self.isthiti = Isthiti::Kaksh(msg.kaksh_kunjika);
        let json = json!({
            "cmd": "random",
            "name": msg.name,
            "kunjika": msg.kunjika
        });
        ctx.text(json.to_string());
    }
}

/// Own Kunjika hash
impl Handler<ms::sansad::WsKunjikaHash> for WsSansad {
    type Result = ();
    fn handle(&mut self, msg: ms::sansad::WsKunjikaHash, ctx: &mut Self::Context) -> Self::Result {
        let json = json!({
            "cmd": "kunjika",
            "kunjika": msg.kunjika
        });
        ctx.text(json.to_string());
    }
}

/// List Vayakti
impl Handler<ms::sansad::WsList> for WsSansad {
    type Result = ();
    fn handle(&mut self, msg: ms::sansad::WsList, ctx: &mut Self::Context) -> Self::Result {
        let json = json!({
            "cmd": "list",
            "vayakti": msg.json
        });
        ctx.text(json.to_string());
    }
}

/// Notify someone got disconnected
impl Handler<ms::sansad::WsDisconnected> for WsSansad {
    type Result = ();
    fn handle(&mut self, msg: ms::sansad::WsDisconnected, ctx: &mut Self::Context) -> Self::Result {
        let json = json!({
            "cmd": "disconnected",
            "name": msg.name,
            "kunjika": msg.kunjika
        });
        ctx.text(json.to_string());
    }
}

/// Send text message
impl Handler<ms::sansad::WsText> for WsSansad {
    type Result = ();
    fn handle(&mut self, msg: ms::sansad::WsText, ctx: &mut Self::Context) -> Self::Result {
        let json = json!({
            "cmd": "text",
            "text": msg.text,
            "reply": msg.reply,
            "kunjika": msg.sender_kunjika, // Sender's kunjuka
            "msg_id": msg.msg_id.to_string()
        });
        ctx.text(json.to_string());
    }
}

/// Send image message
impl Handler<ms::sansad::WsImage> for WsSansad {
    type Result = ();
    fn handle(&mut self, msg: ms::sansad::WsImage, ctx: &mut Self::Context) -> Self::Result {
        let json = json!({
            "cmd": "img",
            "src": msg.src,
            "kunjika": msg.sender_kunjika, // Sender's kunjuka
            "msg_id": msg.msg_id.to_string()
        });
        ctx.text(json.to_string());
    }
}

/// Send reaction message
impl Handler<ms::sansad::WsReaction> for WsSansad {
    type Result = ();
    fn handle(&mut self, msg: ms::sansad::WsReaction, ctx: &mut Self::Context) -> Self::Result {
        let json = json!({
            "cmd": "react",
            "emoji": msg.emoji,
            "kunjika": msg.sender_kunjika, // Sender's kunjuka
            "msg_id": msg.msg_id.to_string()
        });
        ctx.text(json.to_string());
    }
}

/// Send status status
impl Handler<ms::sansad::WsStatus> for WsSansad {
    type Result = ();
    fn handle(&mut self, msg: ms::sansad::WsStatus, ctx: &mut Self::Context) -> Self::Result {
        let json = json!({
            "cmd": "status",
            "status": msg.status,
            "kunjika": msg.sender_kunjika // Sender's kunjuka
        });
        ctx.text(json.to_string());
    }
}

/// Delete messages
impl Handler<ms::sansad::WsDeleteMsg> for WsSansad {
    type Result = ();
    fn handle(&mut self, msg: ms::sansad::WsDeleteMsg, ctx: &mut Self::Context) -> Self::Result {
        let json = json!({
            "cmd": "del",
            "msg_id": msg.msg_id,
            "kunjika": msg.sender_kunjika // Sender's kunjuka
        });
        ctx.text(json.to_string());
    }
}

/// Edit messages
impl Handler<ms::sansad::WsEditMsg> for WsSansad {
    type Result = ();
    fn handle(&mut self, msg: ms::sansad::WsEditMsg, ctx: &mut Self::Context) -> Self::Result {
        let json = json!({
            "cmd": "edit",
            "msg_id": msg.msg_id,
            "text": msg.text,
            "kunjika": msg.sender_kunjika // Sender's kunjuka
        });
        ctx.text(json.to_string());
    }
}

/// Send response ok, error
impl Handler<ms::sansad::WsResponse> for WsSansad {
    type Result = ();
    fn handle(&mut self, msg: ms::sansad::WsResponse, ctx: &mut Self::Context) -> Self::Result {
        let json = json!({
            "cmd": "resp",
            "result": msg.result,
            "message": msg.message
        });
        ctx.text(json.to_string());
    }
}
