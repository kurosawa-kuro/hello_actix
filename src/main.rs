use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

// データモデル
#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

// ハンドラーモジュール
mod handlers {
    use super::*;

    // ヘルスチェック
    #[get("/health")]
    pub async fn health_check() -> impl Responder {
        HttpResponse::Ok().json("OK")
    }

    // ユーザー情報取得
    #[get("/users/{id}")]
    pub async fn get_user(user_id: web::Path<u32>) -> impl Responder {
        let user = User {
            id: *user_id,
            name: "John Doe".to_string(),
            email: "john.doe@example.com".to_string(),
        };
        HttpResponse::Ok().json(user)
    }

    // ユーザー作成
    #[post("/users")]
    pub async fn create_user(new_user: web::Json<User>) -> impl Responder {
        let user = User {
            id: 1,
            name: new_user.name.clone(),
            email: new_user.email.clone(),
        };
        HttpResponse::Created().json(user)
    }
}

// テストモジュール
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};

    #[actix_rt::test]
    async fn test_health_check() {
        let app = test::init_service(
            App::new()
                .service(handlers::health_check)
        ).await;

        let req = test::TestRequest::get()
            .uri("/health")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        assert_eq!(test::read_body(resp).await, "\"OK\"");
    }

    #[actix_rt::test]
    async fn test_get_user() {
        let app = test::init_service(
            App::new()
                .service(handlers::get_user)
        ).await;

        let req = test::TestRequest::get()
            .uri("/users/1")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;
        assert_eq!(body, r#"{"id":1,"name":"John Doe","email":"john.doe@example.com"}"#);
    }

    #[actix_rt::test]
    async fn test_create_user() {
        let app = test::init_service(
            App::new()
                .service(handlers::create_user)
        ).await;

        let new_user = web::Json(User {
            id: 1,
            name: "Jane Doe".to_string(),
            email: "jane.doe@example.com".to_string(),
        });

        let req = test::TestRequest::post()
            .uri("/users")
            .set_json(&new_user)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;
        assert_eq!(body, r#"{"id":1,"name":"Jane Doe","email":"jane.doe@example.com"}"#);
    }
}

// メイン関数
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