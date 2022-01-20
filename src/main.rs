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

use actix_files as fs;
use actix_ratelimit::{MemoryStore, MemoryStoreActor, RateLimiter};
use actix_web::{
    client::{Client, Connector},
    middleware::Logger,
    web, App, Error, HttpRequest, HttpResponse, HttpServer,
};
use actix_web_actors::ws;
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslConnector, SslFiletype, SslMethod};
use std::sync::RwLock;
use ws_sansad::WsSansad;

mod broker_messages;
mod chat_pinnd;
mod config;
mod errors;
mod validator;
mod ws_sansad;

lazy_static! {
    pub static ref SALT: RwLock<String> = RwLock::new("".to_owned());
    pub static ref TENOR_API_KEY: RwLock<Option<String>> = RwLock::new(None);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let (config, config_file) = config::generate();

    *SALT.write().unwrap() = config_file.salt;

    if let Some(key) = config_file.tenor_key {
        *TENOR_API_KEY.write().unwrap() = Some(key);
    }

    let ssl_builder = if config_file.ssl_key.is_some() && config_file.ssl_cert.is_some() {
        generate_ssl_builder(config_file.ssl_key.unwrap(), config_file.ssl_cert.unwrap())
    } else {
        None
    };

    let logger_pattern = config_file.logger_pattern;
    let static_path = config.static_path;

    let server = HttpServer::new(move || {
        let mut app = App::new()
            .wrap(
                RateLimiter::new(MemoryStoreActor::from(MemoryStore::new().clone()).start())
                    .with_interval(std::time::Duration::from_secs(60))
                    .with_max_requests(200),
            )
            .wrap(Logger::new(&logger_pattern))
            .service(web::resource("/ws/").route(web::get().to(ws_index)));

        if TENOR_API_KEY.read().unwrap().is_some() {
            app = app
                .service(web::resource("/gif/{pos}/").route(web::get().to(gif)))
                .service(web::resource("/gif/{pos}/{query}").route(web::get().to(gif)));
        }

        app = app.service(fs::Files::new("/", &static_path).index_file("index.html"));
        app
    });

    if ssl_builder.is_some() && config.port_ssl.is_some() {
        let port = config.port.clone();
        let port_ssl = config.port_ssl.clone().unwrap();
        let redirect_server = HttpServer::new(move || {
            App::new()
                .wrap(
                    RateLimiter::new(MemoryStoreActor::from(MemoryStore::new().clone()).start())
                        .with_interval(std::time::Duration::from_secs(60))
                        .with_max_requests(100),
                )
                .wrap(
                    actix_web_middleware_redirect_https::RedirectHTTPS::with_replacements(&[(
                        port.clone(),
                        port_ssl.clone(),
                    )]),
                )
                .route(
                    "/",
                    web::get().to(|| {
                        HttpResponse::Ok()
                            .content_type("text/plain")
                            .body("Always HTTPS on non-default ports!")
                    }),
                )
        })
        .bind(format!("{}:{}", config.bind_address, config.port))?
        .run();

        let server = server
            .bind_openssl(
                format!("{}:{}", config.bind_address, config.port_ssl.unwrap()),
                ssl_builder.unwrap(),
            )?
            .run();

        tokio::try_join!(redirect_server, server)?;
    } else {
        server
            .bind(format!("{}:{}", config.bind_address, config.port))?
            .run()
            .await?;
    }
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
    let builder = SslConnector::builder(SslMethod::tls()).unwrap();

    let client = Client::builder()
        .connector(Connector::new().ssl(builder.build()).finish())
        .finish();

    let tenor_key = TENOR_API_KEY.read().unwrap();
    let key = match &*tenor_key {
        Some(a) => a.as_str(),
        None => "",
    };

    let url = format!(
        "https://g.tenor.com/v1/search?q={}&key={}&limit=20&media_filter=tinygif&pos={}",
        name.replace(" ", "+"),
        key,
        pos
    );
    let response = client
        .get(url)
        .header("User-Agent", "actix-web/3.0")
        .send()
        .await?
        .body()
        .await?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(response))
}

fn generate_ssl_builder(key: String, cert: String) -> Option<SslAcceptorBuilder> {
    if key != "" && cert != "" {
        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder.set_private_key_file(key, SslFiletype::PEM).unwrap();
        builder.set_certificate_chain_file(cert).unwrap();
        Some(builder)
    } else {
        None
    }
}
