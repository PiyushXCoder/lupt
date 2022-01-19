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

use clap::{ErrorKind as ClapErrorKind, IntoApp, Parser};
// use clap::{App, Arg};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufReader, ErrorKind as IOErrorKind},
    path::PathBuf,
};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub(crate) struct Config {
    /// Path of directory with index.html
    #[clap(short, long, parse(from_os_str), value_name = "FILE")]
    pub(crate) static_path: PathBuf,

    /// Address to bind for server
    #[clap(short, long, value_name = "ADDRESS")]
    pub(crate) bind_address: String,

    /// Port to bind for http server
    #[clap(short, long, value_name = "PORT")]
    pub(crate) port: String,

    /// Port to bind for https (ssl) server
    #[clap(short = 'o', long, value_name = "PORT")]
    pub(crate) port_ssl: Option<String>,

    /// Path to config file
    #[clap(short, long, parse(from_os_str), value_name = "FILE")]
    pub(crate) config_file: PathBuf,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ConfigFile {
    pub(crate) salt: String,
    pub(crate) tenor_key: String,
    pub(crate) ssl_cert: String,
    pub(crate) ssl_key: String,
    pub(crate) logger_pattern: String,
}

const HELP_CONFIG_FILE: &'static str = "Config File is corrupt.

Config file must have following fields
    - salt: Salt for hashing
    - tenor_key: Key of tenor gif api
    - ssl_cert: Path to certificate of ssl
    - ssl_key: Path to private key of ssl
    - logger_pattern: Pattern to make log according to Actix Logger";

pub(crate) fn generate() -> (Config, ConfigFile) {
    let config: Config = Config::parse();
    let config_file = File::open(&config.config_file);
    if let Err(e) = config_file {
        let mut app = Config::into_app();
        match e.kind() {
            IOErrorKind::NotFound => app
                .error(ClapErrorKind::InvalidValue, "Error: Config file is missing")
                .exit(),
            _ => app
                .error(
                    ClapErrorKind::InvalidValue,
                    format!("Error(Config File): {}", e.to_string()),
                )
                .exit(),
        }
    }

    let reader = BufReader::new(config_file.unwrap());
    let json: ConfigFile = match serde_json::from_reader(reader) {
        Ok(read) => read,
        Err(e) => {
            let mut app = Config::into_app();
            app.error(
                ClapErrorKind::InvalidValue,
                format!(
                    "Error(Config File): {}\n\n{}",
                    e.to_string(),
                    HELP_CONFIG_FILE
                ),
            )
            .exit();
        }
    };

    (config, json)
}

// impl Config {
//     pub fn new() -> Self {
//         let matches = App::new("Lupt (लुप्त)")
//             .version(env!("CARGO_PKG_VERSION"))
//             .author(env!("CARGO_PKG_AUTHORS"))
//             .about(env!("CARGO_PKG_DESCRIPTION"))
//             .arg(
//                 Arg::with_name("bind_address")
//                     .short("a")
//                     .long("bind_address")
//                     .value_name("ADDRESS")
//                     .help("Address to bind for server")
//                     .required(true)
//                     .takes_value(true),
//             )
//             .arg(
//                 Arg::with_name("port")
//                     .short("p")
//                     .long("port")
//                     .value_name("PORT")
//                     .help("Port to bind for server")
//                     .required(true)
//                     .takes_value(true),
//             )
//             .arg(
//                 Arg::with_name("port_x")
//                     .short("x")
//                     .long("port_x")
//                     .value_name("PORT")
//                     .help("Port to bind for http if ssl is enabled to redirect to https")
//                     .required(false)
//                     .takes_value(true),
//             )
//             .arg(
//                 Arg::with_name("static_path")
//                     .short("s")
//                     .long("static_path")
//                     .value_name("DIR")
//                     .help("Path of directory with index.html")
//                     .required(true)
//                     .takes_value(true),
//             )
//             .arg(
//                 Arg::with_name("config")
//                     .short("c")
//                     .long("config")
//                     .value_name("FILE")
//                     .help("Path to config file")
//                     .required(true)
//                     .takes_value(true),
//             )
//             .get_matches();

//         let conf = matches.value_of("config").unwrap().to_owned();
//         let conf = std::fs::read_to_string(conf).expect("Failed to read config");

//         let config = serde_json::from_str::<ConfigFile>(&conf).expect(
//             r"
// Config File is corrupt.

// Config file must have following fields
//     - salt: Salt for hashing
//     - tenor_key: Key of tenor gif api
//     - ssl_cert: Path to certificate of ssl
//     - ssl_key: Path to private key of ssl
//     - logger_pattern: Pattern to make log according to Actix Logger
// ",
//         );

//         Config {
//             static_path: matches.value_of("static_path").unwrap().to_owned(),
//             bind_address: matches.value_of("bind_address").unwrap().to_owned(),
//             port: matches.value_of("port").unwrap().to_owned(),
//             port_x: matches.value_of("port_x").unwrap_or("").to_owned(),
//             config,
//         }
//     }
// }
