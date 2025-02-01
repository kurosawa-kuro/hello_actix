use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

// ハンドラーモジュール
mod handlers {
    use super::*;

    #[get("/")]
    pub async fn hello() -> impl Responder {
        HttpResponse::Ok().body("Hello, World!")
    }

    #[post("/echo")]
    pub async fn echo(req_body: String) -> impl Responder {
        HttpResponse::Ok().body(req_body)
    }
}

// テストモジュール
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_rt::test]
    async fn test_hello() {
        let app = test::init_service(
            App::new()
                .service(handlers::hello)
        ).await;

        let req = test::TestRequest::get()
            .uri("/")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;
        assert_eq!(body, "Hello, World!");
    }

    #[actix_rt::test]
    async fn test_echo() {
        let app = test::init_service(
            App::new()
                .service(handlers::echo)
        ).await;

        let req = test::TestRequest::post()
            .uri("/echo")
            .set_payload("test message")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;
        assert_eq!(body, "test message");
    }
}

// メイン関数
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // サーバー設定
    HttpServer::new(|| {
        App::new()
            .service(handlers::hello)
            .service(handlers::echo)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}