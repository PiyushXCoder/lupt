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

impl WsSansad {
    /// Request for joining to random person
    pub async fn join_random(&mut self, val: Value) {
        // Check is already joined
        match self.isthiti {
            Isthiti::None => (),
            Isthiti::VraktigatWaitlist => {
                self.send_ok_response("watchlist");
                return;
            }, Isthiti::Kaksh(_) => return
        }

        let kunjika  = match val.get("kunjika") {
            Some(val ) => val.as_str().unwrap().to_owned(),
            None => {
                self.send_err_response("Invalid request");
                return;
            }
        };
        // kunjika to hash and base64
        let mut m = sha1::Sha1::new();
        m.update(format!("{}{}",kunjika,
            crate::SALT.to_owned()).as_bytes());
        let kunjika = base64::encode(m.digest().bytes())[..8].to_owned();

        let name  = match val.get("name") {
            Some(val ) => val.as_str().unwrap().to_owned(),
            None => {
                self.send_err_response("Invalid request");
                return;
            }
        };
        let tags  = match val.get("tags") {
            Some(val ) => {
                let mut v = Vec::new();
                for x in val.as_str().unwrap().split_ascii_whitespace() {
                    v.push(x.to_owned());
                }
                v
            },
            None => {
                Vec::new()
            }
        };

        // Validate
        if let Some(val ) = validate(vec![vl::NonEmpty, vl::NoSpace, vl::NoHashtag], &kunjika, "Kunjika") {
            self.send_err_response(&val);
            return;
        } else if let Some(val ) = validate(vec![vl::NonEmpty], &name, "Name") {
            self.send_err_response(&val);
            return;
        }

        // request
        let result: Resp = ChatPinnd::from_registry().send(ms::pind::JoinRandom{
            addr: self.addr.clone().unwrap(),
            kunjika: kunjika.to_owned(),
            name,
            tags
        }).await.unwrap();

        match result {
            Resp::Err(err) => self.send_err_response(&err), 
            Resp::Ok =>  {
                self.addr.clone().unwrap().do_send(ms::sansad::WsKunjikaHash{ kunjika: kunjika.clone() });
                self.kunjika = kunjika;
            },
            Resp::None => {
                self.addr.clone().unwrap().do_send(ms::sansad::WsResponse{
                    result: "watch".to_owned() ,
                    message: "Watchlist".to_owned()
                 });
                self.isthiti = Isthiti::VraktigatWaitlist;
                self.addr.clone().unwrap().do_send(ms::sansad::WsKunjikaHash{ kunjika: kunjika.clone() });
                self.kunjika = kunjika
            }
        }
    }

    /// Request for joining to random person
    pub async fn join_random_next(&mut self) {
        // Check is already joined
        let kaksh_kunjika = match &self.isthiti {
            Isthiti::VraktigatWaitlist => {
                self.send_ok_response("watchlist");
                return;
            },
            Isthiti::Kaksh(kaksh_kunjika) => kaksh_kunjika,
            Isthiti::None => {
                self.send_ok_response("Not allowed");
                return;
            }
        };

        // request
        let result: Resp = ChatPinnd::from_registry().send(ms::pind::JoinRandomNext {
            kunjika: self.kunjika.to_owned(),
            kaksh_kunjika: kaksh_kunjika.to_owned(),
        }).await.unwrap();

        match result {
            Resp::Err(err) => self.send_err_response(&err), 
            Resp::None => {
                self.addr.clone().unwrap().do_send(ms::sansad::WsResponse{
                   result: "watch".to_owned() ,
                   message: "Watchlist".to_owned()
                });
                self.isthiti = Isthiti::VraktigatWaitlist;
                self.kunjika = self.kunjika.to_owned()
            }
            _ => ()
        }
    }

    /// Request to join to kaksh
    pub async fn join_kaksh(&mut self, val: Value) {
        // Check is already joined
        match self.isthiti {
            Isthiti::None => (),
            _ => return
        }

        // is vayakti in watch list
        if let Isthiti::VraktigatWaitlist = self.isthiti {
            self.send_ok_response("watchlist");
            return;
        }

        let kunjika  = match val.get("kunjika") {
            Some(val ) => val.as_str().unwrap().to_owned(),
            None => {
                self.send_err_response("Invalid request");
                return;
            }
        };
        // kunjika to hash and base64
        let mut m = sha1::Sha1::new();
        m.update(format!("{}{}",kunjika,
            crate::SALT.to_owned()).as_bytes());
        let kunjika = base64::encode(m.digest().bytes())[..8].to_owned();

        let name  = match val.get("name") {
            Some(val ) => val.as_str().unwrap().to_owned(),
            None => {
                self.send_err_response("Invalid request");
                return;
            }
        };
        let kaksh_kunjika = match val.get("kaksh_kunjika") {
            Some(val ) => val.as_str().unwrap().to_owned(),
            None => {
                self.send_err_response("Invalid request");
                return;
            }
        };
        let length: Option<usize> = match val.get("length") {
            Some(val) => match val.as_i64(){
                    Some(val) => Some(val as usize),
                    None => None
                },
            None => None
        };


        // Validate
        if let Some(val ) = validate(vec![vl::NonEmpty, vl::NoGupt, vl::NoSpace], &kaksh_kunjika, "Kaksh Kunjika") {
            self.send_err_response(&val);
            return;
        } else if let Some(val ) = validate(vec![vl::NonEmpty, vl::NoSpace, vl::NoHashtag], &kunjika, "Kunjika") {
            self.send_err_response(&val);
            return;
        } else if let Some(val ) = validate(vec![vl::NonEmpty], &name, "Name") {
            self.send_err_response(&val);
            return;
        }
        
        // request
        let result: Resp = ChatPinnd::from_registry().send(ms::pind::JoinKaksh {
            kaksh_kunjika: kaksh_kunjika.to_owned(),
            length,
            addr: self.addr.clone().unwrap(),
            kunjika: kunjika.to_owned(),
            name
        }).await.unwrap();


        match result {
            Resp::Err(err) => self.send_err_response(&err), 
            Resp::Ok => {
                self.isthiti = Isthiti::Kaksh(kaksh_kunjika);
                self.addr.clone().unwrap().do_send(ms::sansad::WsKunjikaHash{ kunjika: kunjika.clone() });
                self.kunjika = kunjika;
                self.send_ok_response("joined")
            }
            _ => ()
        }
    }

    /// Request to join to kaksh
    pub async fn list(&mut self) {
        // check if vayakti exist
        if let Isthiti::None = self.isthiti {
            self.send_err_response("Not in any Kaksh");
            return;
        }

        // check if connected to any kaksh
        match &self.isthiti {
            Isthiti::Kaksh(kunjika) => {
                let json: String = ChatPinnd::from_registry().send(ms::pind::List {
                    kaksh_kunjika: kunjika.to_owned()
                }).await.unwrap();

                self.addr.clone().unwrap().do_send(ms::sansad::WsList {
                    json
                })
            },
            _ => {
                self.send_err_response("Kaksh not connected");
                return;
            }
        }
    }

    /// notify leaving
    pub async fn leave_kaksh(&mut self) {
        let kaksh_kunjika = match &self.isthiti {
            Isthiti::Kaksh(val) => Some(val.to_owned()),
            _ => None
        };
        
        Broker::<SystemBroker>::issue_async(ms::pind::LeaveUser {
            kaksh_kunjika,
            kunjika: self.kunjika.to_owned(),
            addr: self.addr.clone().unwrap()
        });
    
        self.isthiti = Isthiti::None;        
        self.send_ok_response("left");
    }
}