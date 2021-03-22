/**
 * rust web api, a fresh start
 * 
 * TODO:
 * - proper async (tokio?)
 * - JWT
 * - insert / delete / update
 * 
 * a fresh set of tips:
 * cargo watch -x run (start in watch mode, thx to cargo-watch crate) 
 * dbg!() for outputting things and their value/pos to the shell...
 */
/*fn main() {
    let _a = 200 * 20;
    let _b = 100;
    let mut _b = 20;
    let mut mystring: &str = "some string a";
    let mut s: String =  mystring.to_owned() + &" b".to_owned();
    let addC: &str = " c";
    let mut reString: &str = &s;
    let combinedString = format!("{}{}", reString, addC);
    dbg!(combinedString);
    dbg!(s);
    dbg!(mystring);
    println!("a fresh start!");
}*/
//pub mod schema;
//pub mod db_connection;
//mod handlers;

mod config;
mod errors;
mod handlers;
pub mod models;
mod schema;
mod middlewares;
// mod repositories;

/*extern crate diesel;
extern crate dotenv;
extern crate serde;
extern crate serde_json;
extern crate serde_derive;
extern crate futures;
extern crate actix;
extern crate actix_web;*/

#[macro_use]
extern crate diesel;

use actix_web::http::ContentEncoding;
use actix_web::{middleware, web, App, HttpServer};
use actix_web::{dev::ServiceRequest, Error};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");


    // create db connection pool
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            //.wrap(middleware::Compress::new(ContentEncoding::Br))
            //.wrap(middleware::Logger::default())
            .route("/customers", web::get().to(handlers::get_customers))
            // .service(web::resource("/").route(web::post().to(index))
            // .service(web::resource("/user").route(web::post().to(handlers::get_user_by_email)))
            //.route("/users/{id}", web::get().to(handlers::get_user_by_id))
            .route("/users", web::post().to(handlers::get_user_by_email))
            //.route("/users/{id}", web::delete().to(handlers::delete_user))
            //.service(web::scope("/user").configure(repositories::user_repository::init_routes))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

/*use actix_web::{
    error, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer,
};
use futures::StreamExt;
use json::JsonValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct MyObj {
    name: String,
    number: i32,
}

/// This handler uses json extractor
async fn index(item: web::Json<MyObj>) -> HttpResponse {
    println!("model: {:?}", &item);
    HttpResponse::Ok().json(item.0) // <- send response
}

/// This handler uses json extractor with limit
async fn extract_item(item: web::Json<MyObj>, req: HttpRequest) -> HttpResponse {
    println!("request: {:?}", req);
    println!("model: {:?}", item);

    HttpResponse::Ok().json(item.0) // <- send json response
}

const MAX_SIZE: usize = 262_144; // max payload size is 256k

/// This handler manually load request payload and parse json object
async fn index_manual(mut payload: web::Payload) -> Result<HttpResponse, Error> {
    // payload is a stream of Bytes objects
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    // body is loaded, now we can deserialize serde-json
    let obj = serde_json::from_slice::<MyObj>(&body)?;
    Ok(HttpResponse::Ok().json(obj)) // <- send response
}

/// This handler manually load request payload and parse json-rust
async fn index_mjsonrust(body: web::Bytes) -> Result<HttpResponse, Error> {
    // body is loaded, now we can deserialize json-rust
    let result = json::parse(std::str::from_utf8(&body).unwrap()); // return Result
    let injson: JsonValue = match result {
        Ok(v) => v,
        Err(e) => json::object! {"err" => e.to_string() },
    };
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(injson.dump()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            .service(web::resource("/extractor").route(web::post().to(index)))
            .service(
                web::resource("/extractor2")
                    .data(web::JsonConfig::default().limit(1024)) // <- limit size of the payload (resource level)
                    .route(web::post().to(extract_item)),
            )
            .service(web::resource("/manual").route(web::post().to(index_manual)))
            .service(web::resource("/mjsonrust").route(web::post().to(index_mjsonrust)))
            .service(web::resource("/").route(web::post().to(index)))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::dev::Service;
    use actix_web::{http, test, web, App};

    #[actix_rt::test]
    async fn test_index() -> Result<(), Error> {
        let mut app = test::init_service(
            App::new().service(web::resource("/").route(web::post().to(index))),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/")
            .set_json(&MyObj {
                name: "my-name".to_owned(),
                number: 43,
            })
            .to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };

        assert_eq!(response_body, r##"{"name":"my-name","number":43}"##);

        Ok(())
    }
}*/