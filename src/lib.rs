use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

// データモデル
#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

// テスト用ユーザーデータ
pub fn test_users() -> Vec<User> {
    vec![
        User {
            id: 1,
            name: "John Doe".to_string(),
            email: "john.doe@example.com".to_string(),
        },
        User {
            id: 2,
            name: "Jane Doe".to_string(),
            email: "jane.doe@example.com".to_string(),
        },
    ]
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
        let users = test_users();
        let user = users.iter().find(|u| u.id == *user_id).unwrap();
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

    // ユーザー情報一覧取得
    #[get("/users")]
    pub async fn get_users() -> impl Responder {
        HttpResponse::Ok().json(test_users())
    }
}