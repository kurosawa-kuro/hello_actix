use actix_web::{test, web, App};
use hello_actix::{handlers, test_users};

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
    assert_eq!(
        body,
        r#"{"id":1,"name":"John Doe","email":"john.doe@example.com"}"#
    );
}

#[actix_rt::test]
async fn test_create_user() {
    let app = test::init_service(
        App::new()
            .service(handlers::create_user)
    ).await;

    let new_user = web::Json(test_users()[0].clone());

    let req = test::TestRequest::post()
        .uri("/users")
        .set_json(&new_user)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    assert_eq!(
        body,
        r#"{"id":1,"name":"John Doe","email":"john.doe@example.com"}"#
    );
}

#[actix_rt::test]
async fn test_get_users() {
    let app = test::init_service(
        App::new()
            .service(handlers::get_users)
    ).await;

    let req = test::TestRequest::get()
        .uri("/users")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    assert_eq!(
        body,
        r#"[{"id":1,"name":"John Doe","email":"john.doe@example.com"},{"id":2,"name":"Jane Doe","email":"jane.doe@example.com"}]"#
    );
}