use actix_web::{delete, get, post, put, web::Data, App, HttpResponse, HttpServer, Responder};
use actix_web_sample::models::user::*; // -(ハイフン) はダメだったが_（アンダーバー）はOK
                                       //use serde_json::json;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub struct AppState {
    db: Pool<Postgres>,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

// モック
#[post("/users")]
async fn create() -> impl Responder {
    HttpResponse::Created()
}

// モック
#[get("/users")]
async fn find_all(state: Data<AppState>) -> impl Responder {
    //HttpResponse::Ok().json(vec![
    //    User {
    //        id: 1,
    //        name: "John".to_string(),
    //        email: "sample@email.com".to_string(),
    //        age: 32,
    //    },
    //    User {
    //        id: 2,
    //        name: "J".to_string(),
    //        email: "sample2@email.com".to_string(),
    //        age: 20,
    //    },
    //])

    match sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&state.db)
        .await
    {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => std::process::exit(1),
    }
}

// モック
#[get("/users/{id}")]
async fn find() -> impl Responder {
    HttpResponse::Ok().json(User {
        id: 1,
        name: "John".to_string(),
        email: "sample@email.com".to_string(),
        age: 32,
    })
}

// モック
#[put("/users/{id}")]
async fn update() -> impl Responder {
    HttpResponse::Ok().json(User {
        id: 1,
        name: "New John".to_string(),
        email: "updatedsample@email.com".to_string(),
        age: 32,
    })
}

// モック
#[delete("/users/{id}")]
async fn delete() -> impl Responder {
    HttpResponse::NoContent()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://user:password@localhost/develop")
        .await
    {
        Ok(pool) => pool,
        Err(_) => std::process::exit(1),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(create)
            .service(find_all)
            .service(find)
            .service(update)
            .service(delete)
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
