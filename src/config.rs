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

use clap::Parser;
use serde::Deserialize;
use std::{fs::File, io::BufReader, path::PathBuf};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub(crate) struct Args {
    /// Path to config file
    #[clap(short, long, value_name = "FILE")]
    pub(crate) config_file: PathBuf,
}

#[derive(Deserialize)]
pub(crate) struct ConfigFile {
    pub(crate) static_dir: PathBuf,

    pub(crate) bind_address: String,
    pub(crate) non_ssl_port: u16,

    pub(crate) ssl_enabled: bool,
    pub(crate) ssl_port: Option<u16>,
    pub(crate) ssl_cert: Option<String>,
    pub(crate) ssl_key: Option<String>,

    pub(crate) logger_pattern: String,
    pub(crate) log_file: PathBuf,

    pub(crate) salt: String,
    pub(crate) tenor_key: Option<String>,
}

pub(crate) fn generate() -> ConfigFile {
    let args: Args = Args::parse();
    let config_file = File::open(&args.config_file)
        .map_err(|e| anyhow!(e))
        .expect("Failed to open config file!");
    let reader = BufReader::new(config_file);
    let json: ConfigFile = serde_json::from_reader(reader)
        .map_err(|e| anyhow!(e))
        .expect("Failed to open config file!");
    json
}

lazy_static! {
    pub(crate) static ref CONFIG: ConfigFile = generate();
}
