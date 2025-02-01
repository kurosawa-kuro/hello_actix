use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

// データモデル
#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

// ハンドラーモジュール
pub mod handlers {
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