
use super::*;

/// send text to everyone
impl Handler<ms::pind::SendText> for ChatPinnd {
    type Result = ();

    fn handle(&mut self, msg: ms::pind::SendText, _: &mut Self::Context) -> Self::Result {
        if let Some(kaksh) = self.kaksh.get_mut(&msg.kaksh_kunjika) {
            kaksh.last_message_id += 1;
            let msg_id = kaksh.last_message_id;
            kaksh.loog.iter().for_each(|c| {
                c.addr.do_send(ms::sansad::WsText {
                    sender_kunjika: msg.kunjika.to_owned(),
                    text: msg.text.to_owned(),
                    reply: msg.reply.to_owned(),
                    msg_id
                });
            });
        }
    }
}

/// send image to everyone
impl Handler<ms::pind::SendImage> for ChatPinnd {
    type Result = ();

    fn handle(&mut self, msg: ms::pind::SendImage, _: &mut Self::Context) -> Self::Result {
        if let Some(kaksh) = self.kaksh.get_mut(&msg.kaksh_kunjika) {
            kaksh.last_message_id += 1;
            let msg_id = kaksh.last_message_id;
            kaksh.loog.iter().for_each(|c| {
                c.addr.do_send(ms::sansad::WsImage {
                    sender_kunjika: msg.kunjika.to_owned(),
                    src: msg.src.to_owned(),
                    msg_id
                });
            });
        }
    }
}

/// send status to everyone
impl Handler<ms::pind::SendStatus> for ChatPinnd {
    type Result = ();

    fn handle(&mut self, msg: ms::pind::SendStatus, _: &mut Self::Context) -> Self::Result {
        if let Some(kaksh) = self.kaksh.get(&msg.kaksh_kunjika) {
            kaksh.loog.iter().for_each(|c| {
                if c.kunjika == msg.kunjika {
                    return;
                }
                c.addr.do_send(ms::sansad::WsStatus {
                    sender_kunjika: msg.kunjika.to_owned(),
                    status: msg.status.to_owned(),
                });
            });
        }
    }
}