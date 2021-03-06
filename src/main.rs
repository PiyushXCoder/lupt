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

use actix_web::{
    App, Error, HttpRequest, HttpResponse, HttpServer, middleware::Logger, web,
    client::{Client, Connector}
};
use actix_files as fs;
use actix_web_actors::ws;
use actix_ratelimit::{RateLimiter, MemoryStore, MemoryStoreActor};
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslConnector, SslFiletype, SslMethod};
use ws_sansad::WsSansad;
use std::sync::RwLock;

mod config;
mod errors;
mod broker_messages;
mod ws_sansad;
mod chat_pinnd;
mod validator;

lazy_static! {
    pub static ref SALT: RwLock<String> = RwLock::new(String::new());
    pub static ref TENOR_API_KEY: RwLock<String> = RwLock::new(String::new());
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let config = config::Config::new();

    *SALT.write().unwrap() = config.config.salt;
    *TENOR_API_KEY.write().unwrap() = config.config.tenor_key;

    let ssl_builder = generate_ssl_builder(config.config.ssl_key, config.config.ssl_cert);
    let logger_pattern = config.config.logger_pattern;
    let static_path = config.static_path;

    let mut redirect = None;
    let port_x = config.port_x.clone();
    let port = config.port.clone();
    if ssl_builder.is_some() && config.port_x != "" {
        redirect = Some(HttpServer::new(move || {
            App::new()
            .wrap(
                RateLimiter::new(
                MemoryStoreActor::from(MemoryStore::new().clone()).start())
                    .with_interval(std::time::Duration::from_secs(60))
                    .with_max_requests(100)
            )
            .wrap(actix_web_middleware_redirect_https::RedirectHTTPS::with_replacements(&[(port_x.clone(), port.clone())]))
            .route("/", web::get().to(|| HttpResponse::Ok()
                                            .content_type("text/plain")
                                            .body("Always HTTPS on non-default ports!")))
        })
        .bind(format!("{}:{}", config.bind_address, config.port_x))?
        .run());
    }

    let server = HttpServer::new(move || {
        App::new()
        .wrap(
            RateLimiter::new(
            MemoryStoreActor::from(MemoryStore::new().clone()).start())
                .with_interval(std::time::Duration::from_secs(60))
                .with_max_requests(200)
        )
        .wrap(Logger::new(&logger_pattern))
        .service(web::resource("/ws/").route(web::get().to(ws_index)))
        .service(web::resource("/gif/{pos}/").route(web::get().to(gif)))
        .service(web::resource("/gif/{pos}/{query}").route(web::get().to(gif)))
        .service(fs::Files::new("/", &static_path).index_file("index.html"))
    });
    
    if ssl_builder.is_some() && config.port_x != "" { 
        let srv = server.bind_openssl(format!("{}:{}", config.bind_address, config.port), ssl_builder.unwrap())?.run();
        tokio::try_join!(redirect.unwrap(),  srv)?;
    } else {
        server.bind(format!("{}:{}", config.bind_address, config.port))?.run().await?;
    }
    
    Ok(())
}

async fn ws_index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(WsSansad::new(), &req, stream)
}

async fn gif(req: HttpRequest) -> Result<HttpResponse, Error> {
    let name = req.match_info().get("query").unwrap_or("");
    let mut pos = req.match_info().get("pos").unwrap_or("");
    if pos == "_" { pos = "" }
    let builder = SslConnector::builder(SslMethod::tls()).unwrap();

    let client = Client::builder()
        .connector(Connector::new().ssl(builder.build()).finish())
        .finish();

    
    let url = format!("https://g.tenor.com/v1/search?q={}&key={}&limit=20&media_filter=tinygif&pos={}", name.replace(" ", "+"), TENOR_API_KEY.read().unwrap(), pos);
    let response = client.get(url)
        .header("User-Agent", "actix-web/3.0")
        .send()     
        .await?
        .body()
        .await?;

    Ok(HttpResponse::Ok().content_type("application/json").body(response))
}

fn generate_ssl_builder(key: String, cert: String) -> Option<SslAcceptorBuilder> {
    if key != "" && cert != "" {
        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder
            .set_private_key_file(key, SslFiletype::PEM)
            .unwrap();
        builder.set_certificate_chain_file(cert).unwrap();
        Some(builder)
    } else {
        None
    }
}