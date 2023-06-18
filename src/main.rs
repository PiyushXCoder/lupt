/*
    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

//! Lupt chat
//! Chat Website to have group chat and stranger's chat both
//!
//! Structure of how program work flow
//!
//!           |--> ws_sansad1 <----\
//! ws_index -|--> ws_sansad2 <---- \ chat_pind
//!           |--> ws_sansad3 <---- /
//!           |--> ws_sansad4 <----/
//!

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate anyhow;

use actix_files as fs;
use actix_web::{middleware::Logger, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use awc::Client;
use config::CONFIG;
use rustls::{Certificate, PrivateKey, ServerConfig};
use std::fs::File;
use ws_sansad::WsSansad;

mod broker_messages;
mod chat_pinnd;
mod config;
mod errors;
mod validator;
mod ws_sansad;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    lazy_static::initialize(&CONFIG);

    let main_server = HttpServer::new(move || {
        let mut app = App::new()
            .wrap(Logger::new(&CONFIG.logger_pattern))
            .service(web::resource("/ws/").route(web::get().to(ws_index)));

        if let Some(key) = &CONFIG.tenor_key {
            if key.len() > 0 {
                app = app
                    .service(web::resource("/gif/{pos}/").route(web::get().to(gif)))
                    .service(web::resource("/gif/{pos}/{query}").route(web::get().to(gif)));
            }
        }

        app = app.service(fs::Files::new("/", &CONFIG.static_dir_path).index_file("index.html"));
        app
    });

    let main_server = if CONFIG.allow_ssl.unwrap_or(false) {
        main_server
            .bind_rustls(
                &CONFIG.bind_address,
                gen_rustls_server_config(
                    CONFIG.ssl_key.clone().unwrap(),
                    CONFIG.ssl_cert.clone().unwrap(),
                ),
            )?
            .run()
    } else {
        main_server.bind(&CONFIG.bind_address)?.run()
    };

    main_server.await
}

fn gen_rustls_server_config(key: String, cert: String) -> ServerConfig {
    let mut br = std::io::BufReader::new(File::open(cert).unwrap());
    let certs = rustls_pemfile::certs(&mut br)
        .unwrap()
        .iter()
        .map(|a| Certificate(a.to_owned()))
        .collect::<Vec<Certificate>>();

    let mut br = std::io::BufReader::new(File::open(key).unwrap());
    let private_key = rustls_pemfile::ec_private_keys(&mut br).unwrap_or(
        rustls_pemfile::rsa_private_keys(&mut br)
            .unwrap_or(rustls_pemfile::pkcs8_private_keys(&mut br).unwrap()),
    );

    let private_key = private_key.get(0).unwrap();

    let private_key = PrivateKey(private_key.to_owned());

    let mut config = ServerConfig::builder()
        .with_safe_default_cipher_suites()
        .with_safe_default_kx_groups()
        .with_safe_default_protocol_versions()
        .map_err(|e| anyhow!(e))
        .expect("Build TLS!")
        .with_no_client_auth()
        .with_single_cert(certs, private_key)
        .map_err(|e| anyhow!(e))
        .expect("Add TLS certificates!");
    config
}

async fn ws_index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(WsSansad::new(), &req, stream)
}

async fn gif(req: HttpRequest) -> Result<HttpResponse, Error> {
    let name = req.match_info().get("query").unwrap_or("");
    let mut pos = req.match_info().get("pos").unwrap_or("");
    if pos == "_" {
        pos = ""
    }

    let client = Client::default();

    let tenor_key = CONFIG.tenor_key.clone().unwrap();

    let url = if name != "" {
        format!(
        "https://tenor.googleapis.com/v2/search?q={}&key={}&limit=20&media_filter=tinygif&pos={}",
        name.replace(" ", "+"),
        tenor_key,
        pos
        )
    } else {
        format!(
            "https://tenor.googleapis.com/v2/featured?key={}&limit=20&media_filter=tinygif&pos={}",
            tenor_key, pos
        )
    };

    let response = client
        .get(url)
        .insert_header(("User-Agent", "actix-web/3.0"))
        .send()
        .await
        .unwrap()
        .body()
        .await
        .unwrap(); // need handle errors

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(response))
}
