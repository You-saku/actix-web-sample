use actix_web::{get, post, put, delete, web, App, HttpResponse, HttpServer, Responder};
use actix_web_sample::models::user::*; // -(ハイフン) はダメだったが_（アンダーバー）はOK

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

// モック
#[post("/users")]
async fn create() -> impl Responder {
    HttpResponse::Created()
}

// モック
#[get("/users")]
async fn find_all() -> impl Responder {
    HttpResponse::Ok().json(vec![
        User {
            id: 1,
            name : "John".to_string(),
            email: "sample@email.com".to_string(),
            age: 32
        },
        User {
            id: 2,
            name : "J".to_string(),
            email: "sample2@email.com".to_string(),
            age: 20
        },
    ])
}

// モック
#[get("/users/{id}")]
async fn find() -> impl Responder {
    HttpResponse::Ok().json(
        User {
            id: 1,
            name : "John".to_string(),
            email: "sample@email.com".to_string(),
            age: 32
        },
    )
}

// モック
#[put("/users/{id}")]
async fn update() -> impl Responder {
    HttpResponse::Ok().json(
        User {
            id: 1,
            name : "New John".to_string(),
            email: "updatedsample@email.com".to_string(),
            age: 32
        },
    )
}

// モック
#[delete("/users/{id}")]
async fn delete() -> impl Responder {
    HttpResponse::NoContent()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(create)
            .service(find_all)
            .service(find)
            .service(update)
            .service(delete)
            .service(hello)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
