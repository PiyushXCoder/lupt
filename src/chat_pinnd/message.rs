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

/// send Reaction to everyone
impl Handler<ms::pind::SendReaction> for ChatPinnd {
    type Result = ();

    fn handle(&mut self, msg: ms::pind::SendReaction, _: &mut Self::Context) -> Self::Result {
        if let Some(kaksh) = self.kaksh.get_mut(&msg.kaksh_kunjika) {
            kaksh.loog.iter().for_each(|c| {
                c.addr.do_send(ms::sansad::WsReaction {
                    sender_kunjika: msg.kunjika.to_owned(),
                    emoji: msg.emoji.to_owned(),
                    msg_id: msg.msg_id.to_owned()
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

/// send delete messages for everyone
impl Handler<ms::pind::DeleteMsg> for ChatPinnd {
    type Result = ();

    fn handle(&mut self, msg: ms::pind::DeleteMsg, _: &mut Self::Context) -> Self::Result {
        if let Some(kaksh) = self.kaksh.get_mut(&msg.kaksh_kunjika) {
            kaksh.loog.iter().for_each(|c| {
                c.addr.do_send(ms::sansad::WsDeleteMsg {
                    sender_kunjika: msg.kunjika.to_owned(),
                    msg_id: msg.msg_id.clone()
                });
            });
        }
    }
}


/// send edit messages for everyone
impl Handler<ms::pind::EditMsg> for ChatPinnd {
    type Result = ();

    fn handle(&mut self, msg: ms::pind::EditMsg, _: &mut Self::Context) -> Self::Result {
        if let Some(kaksh) = self.kaksh.get_mut(&msg.kaksh_kunjika) {
            kaksh.loog.iter().for_each(|c| {
                c.addr.do_send(ms::sansad::WsEditMsg {
                    sender_kunjika: msg.kunjika.to_owned(),
                    msg_id: msg.msg_id.to_owned(),
                    text: msg.text.to_owned()
                });
            });
        }
    }
}