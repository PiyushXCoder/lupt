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
use rustls_pemfile::{certs, pkcs8_private_keys};
use simplelog::*;
use std::fs::File;
use std::io::BufReader;
use ws_sansad::WsSansad;

mod broker_messages;
mod chat_pinnd;
mod config;
mod errors;
mod validator;
mod ws_sansad;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    lazy_static::initialize(&CONFIG);

    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Warn,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::options()
                .write(true)
                .open(&CONFIG.log_file)
                .unwrap_or_else(|_| {
                    println!("Creating new log file");
                    File::create(&CONFIG.log_file).unwrap()
                }),
        ),
    ])
    .unwrap();

    let server = HttpServer::new(move || {
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

        app = app.service(fs::Files::new("/", &CONFIG.static_dir).index_file("index.html"));
        app
    });

    if CONFIG.ssl_enabled {
        let server = server
            .bind_rustls(
                format!("{}:{}", CONFIG.bind_address, CONFIG.ssl_port.unwrap()),
                load_rustls_config(
                    CONFIG.ssl_key.clone().unwrap(),
                    CONFIG.ssl_cert.clone().unwrap(),
                ),
            )?
            .run();
        let redirect_server = create_redirect_server(
            CONFIG.ssl_port.unwrap(),
            CONFIG.non_ssl_port,
            &CONFIG.bind_address,
        );
        let (r1, r2) = tokio::join!(server, redirect_server);
        r1.unwrap();
        r2.unwrap();
    } else {
        server
            .bind(format!("{}:{}", CONFIG.bind_address, CONFIG.non_ssl_port))?
            .run()
            .await?;
    }

    Ok(())
}

fn load_rustls_config(key: String, cert: String) -> rustls::ServerConfig {
    // Load key files
    let cert_file = &mut BufReader::new(File::open(cert).unwrap());
    let key_file = &mut BufReader::new(File::open(key).unwrap());

    // Parse the certificate and set it in the configuration
    let cert_chain = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(|a: Vec<u8>| rustls::Certificate(a))
        .collect::<Vec<Certificate>>();
    let key = pkcs8_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(|a: Vec<u8>| rustls::PrivateKey(a))
        .collect::<Vec<PrivateKey>>()
        .first()
        .unwrap()
        .to_owned();

    // Create configuration
    ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert_chain, key)
        .unwrap()
}

async fn create_redirect_server(
    ssl_port: u16,
    non_ssl_port: u16,
    bind_address: &str,
) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new().wrap(
            actix_web_middleware_redirect_https::RedirectHTTPS::with_replacements(&[(
                non_ssl_port.to_string(),
                ssl_port.to_string(),
            )]),
        )
    })
    .bind(format!("{}:{}", bind_address, non_ssl_port))?
    .run()
    .await
    .unwrap();
    Ok(())
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
