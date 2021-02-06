use actix::Addr;
use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer, web};
use actix_files as fs;
use actix_web_actors::ws;
use chat_pinnd::ChatPinnd;
use ws_sansad::WsSansad;

mod config;
mod ws_sansad;
mod chat_pinnd;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::Config::new();
    let addr =  web::Data::new(ChatPinnd::start());
    let static_path = config.static_path;
    HttpServer::new(move || {
        App::new()
        .app_data(addr.clone())
        .service(web::resource("/ws/").route(web::get().to(ws_index)))
        .service(fs::Files::new("/", &static_path).index_file("index.html"))
    })
    .bind(config.bind_address)?
    .run()
    .await
}

async fn ws_index(req: HttpRequest, stream: web::Payload, pinnd: web::Data<Addr<ChatPinnd>>) -> Result<HttpResponse, Error> {
    let (addr, resp) = ws::start_with_addr(WsSansad::new(pinnd), &req, stream)?;
    addr.do_send(ws_sansad::SelfAddr(addr.clone()));
    Ok(resp)
}
