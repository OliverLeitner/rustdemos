// src/handlers.rs
use super::models::{User, Customer};
use super::schema::customers::dsl::*;
use super::schema::users::dsl::*;
use super::Pool;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use actix_web::{web, Error, HttpResponse};
use diesel::dsl::{delete, insert_into};
use serde::{Deserialize, Serialize};
use std::vec::Vec;
use crate::diesel::ExpressionMethods;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserByEmailObject {
    name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputCustomer {
    pub contactFirstName: String,
    pub contactLastName: String,
    pub customerName: String,
}

pub fn alittleTest() {
    dbg!("somestuff");
}

// Handler for GET /customers
pub async fn get_customers(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_customers(db))
        .await
        .map(|customer| HttpResponse::Ok().json(customer))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

// gets userdata from the db, already working
pub async fn get_user(db: web::Data<Pool>, item: web::Json<GetUserByEmailObject>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || find_user(db, item))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn get_all_customers(pool: web::Data<Pool>) -> Result<Vec<Customer>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let all_customers = customers.load::<Customer>(&conn)?;
    Ok(all_customers)
}

fn find_user(pool: web::Data<Pool>, item: web::Json<GetUserByEmailObject>) -> Result<User, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let name = &item.0.name;
    let result_item = users.filter(user_name.eq(name))
        .or_filter(user_email.eq(name))
        .get_result::<User>(&conn);
    match &result_item {
        Ok(s) => println!("{:?}", s),
        Err(e) => println!("{}", e)
    }
    Ok(result_item.unwrap())
}
