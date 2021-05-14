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

use clap::{App, Arg};
use serde::{Deserialize, Serialize};

pub struct Config {
    pub static_path: String,
    pub bind_address: String,
    pub config: ConfigFile
}

#[derive(Serialize, Deserialize)]
pub struct ConfigFile {
    pub salt: String,
    pub tenor_key: String,
    pub ssl_cert: String,
    pub ssl_key: String,
    pub logger_pattern: String
}

impl Config {
    pub fn new() -> Self {
        let matches = App::new("Lupt (लुप्त)")
            .version(env!("CARGO_PKG_VERSION"))
            .author(env!("CARGO_PKG_AUTHORS"))
            .about(env!("CARGO_PKG_DESCRIPTION"))
            .arg(Arg::with_name("static_path")
                .short("s")
                .long("static_path")
                .value_name("DIR")
                .help("Path of directory with index.html")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("bind_address")
                .short("a")
                .long("bind_address")
                .value_name("ADDRESS")
                .help("Address to bind for server")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Path to config file")
                .required(true)
                .takes_value(true))
            .get_matches();

            let conf = matches.value_of("config").unwrap().to_owned();
            let conf = std::fs::read_to_string(conf).expect("Failed to read config");
            
            let config = serde_json::from_str::<ConfigFile>(&conf).expect(r"
Config File is corrupt.

Config file must have following fields
    - salt: Salt for hashing
    - tenor_key: Key of tenor gif api
    - ssl_cert: Path to certificate of ssl
    - ssl_key: Path to private key of ssl
    - logger_pattern: Pattern to make log according to Actix Logger
");


        Config {
            static_path: matches.value_of("static_path").unwrap().to_owned(),
            bind_address: matches.value_of("bind_address").unwrap().to_owned(),
            config
        }
    }
}