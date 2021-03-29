mod config;
mod errors;
pub mod handlers;
pub mod models;
mod schema;
mod middlewares; // log logger, login for users

#[macro_use]
extern crate diesel;

use actix_web::http::ContentEncoding;
use actix_web::{middleware, web, App, HttpServer};
use actix_web::{dev::ServiceRequest, Error};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use middlewares::*; // logging, logon
pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    // logging
    let myLogger = log::LoggingService {
        filename_suffix: "logfile".to_string(),
        filename_prefix: hostname::get()?.into_string().unwrap(), // os string to string without the baggage
        file_log_path: std::env::var("WEBAPI_LOGDIR").expect("WEBAPI_LOGDIR not set")
    };
    myLogger.logger();

    // static settings for user login checks
    let settings = std::sync::Arc::new(login::UserLoginService {
        secret_salt: std::env::var("SECRET_KEY").expect("SECRET_KEY not set"),
        table_name: std::env::var("USER_COLLECTION_NAME").expect("USER_COLLECTION_NAME not set")
    });

    // db con
    let database_con = std::env::var("DATABASE_CONNECTION").expect("DATABASE_CONNECTION must be set");
    let database_name = std::env::var("DATABASE_NAME").expect("DATABASE_NAME needs to be set");
    let odbc_string = format!("{}/{}", database_con, database_name);

    //
    //static
    //MyClassName:myfunctionormethodname
    //
    //non-static (does not work in actix-web)
    //let mynode = MyClassName {}
    //mynode.myfunctiuonormethodname

    // create db connection pool
    let manager = ConnectionManager::<MysqlConnection>::new(odbc_string);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .data(settings.clone()) // passing data from class to the same class again, looping
            //.wrap(middleware::Compress::new(ContentEncoding::Br))
            .wrap(middleware::Logger::default())
            .route("/customers", web::get().to(handlers::get_customers))
            // var_dump(something), console.log(myobject)
            .route("/login", web::post().to(login::UserLoginService::index)) // inside of this class, call the get_user
            // .service(web::resource("/").route(web::post().to(index))
            // .service(web::resource("/user").route(web::post().to(handlers::get_user_by_email)))
            //.route("/users/{id}", web::get().to(handlers::get_user_by_id))
            .route("/users", web::post().to(handlers::get_user)) // works just fine
            //.route("/users/{id}", web::delete().to(handlers::delete_user))
            //.service(web::scope("/user").configure(repositories::user_repository::init_routes))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
