use actix_utils::mpsc;
use actix_web::http::{header, Method, StatusCode};
use actix_web::{
    error, get, guard, middleware, web, App, Error, HttpRequest, HttpResponse,
    HttpServer, Result, Responder
};
use std::vec::Vec;
use std::sync::Arc;
use serde::{Deserialize, Serialize};

use crate::handlers; // access a pub mod "from above"
use crate::handlers::{GetUserByEmailObject};
use crate::Pool;

#[derive(Debug, Clone)]
pub struct UserLoginService {
    pub secret_salt: String,
    pub table_name: String
}

impl UserLoginService {
    pub fn requestLogin(self: &Self) -> bool {
        println!("some stuff {}", "salty");
        return true; // UserLoginService { secret_salt: self.secret_salt, table_name: self.table_name };
    }

    /// Use the `Data<T>` extractor to access data in a handler.
    pub async fn index(db: web::Data<Pool>, data: web::Data<Arc<UserLoginService>>, item: web::Json<GetUserByEmailObject>) -> impl Responder {
        let mut data = data.requestLogin();
        handlers::get_user(db, item); // TODO: return this
        // println!("{}", item.name);
        // data.counter += 1;
        // println!("{}", data);
        // handlers::alittleTest();
        HttpResponse::Ok()
    }

    //pub async fn get_user(db: web::Data<Pool>, item: web::Json<GetUserByEmailObject>) -> Result<HttpResponse, Error> {

}
