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

use clap::{App, Arg};
pub struct Config {
    pub static_path: String,
    pub bind_address: String
}

impl Config {
    pub fn new() -> Self {
        let bind_address = std::env::var("URL");
        let static_path = std::env::var("STATIC_PATH");

        if bind_address.is_ok() && static_path.is_ok() {
            return Config {
                static_path: static_path.unwrap(),
                bind_address: bind_address.unwrap()
            };
        }

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
            .get_matches();

        Config {
            static_path: matches.value_of("static_path").unwrap().to_owned(),
            bind_address: matches.value_of("bind_address").unwrap().to_owned()
        }
    }
}