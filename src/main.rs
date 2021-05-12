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
use openssl::ssl::{SslConnector, SslMethod};
use ws_sansad::WsSansad;

mod config;
mod errors;
mod broker_messages;
mod ws_sansad;
mod chat_pinnd;
mod validator;

lazy_static! {
    pub static ref SALT: String = std::env::var("SALT").unwrap();
    pub static ref TENOR_API_KEY: String = std::env::var("TENOR_API_KEY").unwrap();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let store = MemoryStore::new();
    let config = config::Config::new();
    let static_path = config.static_path;
    HttpServer::new(move || {
        App::new()
        .wrap(
            RateLimiter::new(
            MemoryStoreActor::from(store.clone()).start())
                .with_interval(std::time::Duration::from_secs(60))
                .with_max_requests(200)
        )
        .wrap(Logger::new("%t [%{x-forwarded-for}i] %s %{User-Agent}i %r"))
        .service(web::resource("/ws/").route(web::get().to(ws_index)))
        .service(web::resource("/gif/{pos}/").route(web::get().to(gif)))
        .service(web::resource("/gif/{pos}/{query}").route(web::get().to(gif)))
        .service(fs::Files::new("/", &static_path).index_file("index.html"))
    })
    .bind(config.bind_address)?
    .run()
    .await
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

    
    let url = format!("https://g.tenor.com/v1/search?q={}&key={}&limit=20&media_filter=tinygif&pos={}", name.replace(" ", "+"), TENOR_API_KEY.to_owned(), pos);
    let response = client.get(url)
        .header("User-Agent", "actix-web/3.0")
        .send()     
        .await?
        .body()
        .await?;

    Ok(HttpResponse::Ok().content_type("application/json").body(response))
}