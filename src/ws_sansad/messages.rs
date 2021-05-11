use super::*;

impl WsSansad {

    /// send text to vayakti in kaksh
    pub async fn send_text(&mut self, val: Value) {
        // check if vayakti exist
        if let Isthiti::None = self.isthiti {
            self.send_err_response("Not in any Kaksh");
            return;
        }

        // check if connected to any kaksh
        match self.isthiti {
            Isthiti::Kaksh(_) => (),
            _ => {
                self.send_err_response("Kaksh not connected");
                return;
            }
        }

        // sent text
        let text = match val.get("text") {
            Some(val) => val,
            None => {
                self.send_err_response("Invalid request");
                return;
            }
        }.as_str().unwrap().to_owned();

        let reply: Option<String> = match val.get("reply") {
            Some(val) => Some(val.as_str().unwrap().to_owned()),
            None => None
        };

        let kaksh_kunjika = match &self.isthiti {
            Isthiti::Kaksh(kaksh_kunjika) => {
                kaksh_kunjika.to_owned()
            }, _ => {
                return;
            }
        };
        Broker::<SystemBroker>::issue_async(ms::pind::SendText {
            kaksh_kunjika,
            kunjika: self.kunjika.to_owned(),
            text,
            reply
        });
    }

    /// send status to vayakti in kaksh
    pub async fn send_status(&mut self, val: Value) {
        // check if vayakti exist
        if let Isthiti::None = self.isthiti {
            self.send_err_response("Not in any Kaksh");
            return;
        }

        // check if connected to any kaksh
        match self.isthiti {
            Isthiti::Kaksh(_) => (),
            _ => {
                self.send_err_response("Kaksh not connected");
                return;
            }
        }

        // sent status
        let status = match val.get("status") {
            Some(val) => val,
            None => {
                self.send_err_response("Invalid request");
                return;
            }
        }.as_str().unwrap().to_owned();
        let kaksh_kunjika = match &self.isthiti {
            Isthiti::Kaksh(kaksh_kunjika) => {
                kaksh_kunjika.to_owned()
            }, _ => {
                return;
            }
        };
        Broker::<SystemBroker>::issue_async(ms::pind::SendStatus {
            kaksh_kunjika,
            kunjika: self.kunjika.to_owned(),
            status
        });
    }

    /// send image to vayakti in kaksh
    pub async fn send_image(&mut self, val: Value) {
        // check if vayakti exist
        if let Isthiti::None = self.isthiti {
            self.send_err_response("Not in any Kaksh");
            return;
        }

        // check if connected to any kaksh
        match self.isthiti {
            Isthiti::Kaksh(_) => (),
            _ => {
                self.send_err_response("Kaksh not connected");
                return;
            }
        }

        // image src
        let src = match val.get("src") {
            Some(val) => val,
            None => {
                self.send_err_response("Invalid request");
                return;
            }
        }.as_str().unwrap().to_owned();

        let kaksh_kunjika = match &self.isthiti {
            Isthiti::Kaksh(kaksh_kunjika) => {
                kaksh_kunjika.to_owned()
            }, _ => {
                return;
            }
        };
        Broker::<SystemBroker>::issue_async(ms::pind::SendImage {
            kaksh_kunjika,
            kunjika: self.kunjika.to_owned(),
            src
        });
    }

    /// delete text to vayakti in kaksh
    pub async fn delete_msg(&mut self, val: Value) {
        // check if vayakti exist
        if let Isthiti::None = self.isthiti {
            self.send_err_response("Not in any Kaksh");
            return;
        }

        // check if connected to any kaksh
        match self.isthiti {
            Isthiti::Kaksh(_) => (),
            _ => {
                self.send_err_response("Kaksh not connected");
                return;
            }
        }

        // image src
        let mut msg_id = vec![];

        let ids = match val.get("msg_id") {
            Some(val) => val,
            None => {
                self.send_err_response("Invalid request");
                return;
            }
        }.as_array().unwrap();
        
        for id in ids {
            msg_id.push(id.as_str().unwrap().to_owned());
        }
        drop(ids);

        let kaksh_kunjika = match &self.isthiti {
            Isthiti::Kaksh(kaksh_kunjika) => {
                kaksh_kunjika.to_owned()
            }, _ => {
                return;
            }
        };

        Broker::<SystemBroker>::issue_async(ms::pind::DeleteMsg {
            kaksh_kunjika,
            kunjika: self.kunjika.to_owned(),
            msg_id
        });
    }

    /// send text to vayakti in kaksh
    pub async fn edit_msg(&mut self, val: Value) {
        // check if vayakti exist
        if let Isthiti::None = self.isthiti {
            self.send_err_response("Not in any Kaksh");
            return;
        }

        // check if connected to any kaksh
        match self.isthiti {
            Isthiti::Kaksh(_) => (),
            _ => {
                self.send_err_response("Kaksh not connected");
                return;
            }
        }

        // edit text
        let text = match val.get("text") {
            Some(val) => val,
            None => {
                self.send_err_response("Invalid request");
                return;
            }
        }.as_str().unwrap().to_owned();

        // msg_id
        let msg_id = match val.get("msg_id") {
            Some(val) => val,
            None => {
                self.send_err_response("Invalid request");
                return;
            }
        }.as_str().unwrap().to_owned();

        let kaksh_kunjika = match &self.isthiti {
            Isthiti::Kaksh(kaksh_kunjika) => {
                kaksh_kunjika.to_owned()
            }, _ => {
                return;
            }
        };
        Broker::<SystemBroker>::issue_async(ms::pind::EditMsg {
            kaksh_kunjika,
            kunjika: self.kunjika.to_owned(),
            text,
            msg_id
        });
    }
}
