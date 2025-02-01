use actix_web::{App, HttpServer};
use hello_actix::handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // サーバー設定
    HttpServer::new(|| {
        App::new()
            .service(handlers::health_check)
            .service(handlers::get_user)
            .service(handlers::create_user)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}