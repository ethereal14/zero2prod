use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::net::TcpListener;

// 注意不同的函数签名
// 在正常情况下返回Server,并删除了async关键字
// 没有进行.await调用，所以不再需要它了
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server: Server =
        HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
            .listen(listener)?
            .run();
    Ok(server)
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
