
use super::*;

/// send text message
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

/// send text message
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


/// send text status
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

/// send response ok, error
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

/// notify someone got disconnected
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

/// notify got connected to random person
impl Handler<ms::sansad::WsConnectedRandom> for WsSansad {
    type Result = ();
    fn handle(&mut self, msg: ms::sansad::WsConnectedRandom, ctx: &mut Self::Context) -> Self::Result {
        self.isthiti = Isthiti::Kaksh(msg.kaksh_kunjika);
        let json = json!({
            "cmd": "random",
            "name": msg.name,
            "kunjika": msg.kunjika
        });
        ctx.text(json.to_string());
    }
}
