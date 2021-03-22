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
    email: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputCustomer {
    pub contactFirstName: String,
    pub contactLastName: String,
    pub customerName: String,
}

// Handler for GET /customers
pub async fn get_customers(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_customers(db))
        .await
        .map(|customer| HttpResponse::Ok().json(customer))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

// get user by email
pub async fn get_user_by_email(db: web::Data<Pool>, item: web::Json<GetUserByEmailObject>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || find_user_with_email(db, item))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}


fn get_all_customers(pool: web::Data<Pool>) -> Result<Vec<Customer>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let all_customers = customers.load::<Customer>(&conn)?;
    Ok(all_customers)
}

fn find_user_with_email(pool: web::Data<Pool>, item: web::Json<GetUserByEmailObject>) -> Result<User, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let result_item = users.filter(user_email.eq(item.0.email)).get_result::<User>(&conn);
    match &result_item {
        Ok(s) => println!("{:?}", s),
        Err(e) => println!("{}", e)
    }
    Ok(result_item.unwrap())
}

// Handler for GET /users/{id}
/*pub async fn get_customer_by_id(
    db: web::Data<Pool>,
    customer_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_get_customer_by_id(db, customer_id.into_inner()))
            .await
            .map(|customer| HttpResponse::Ok().json(customer))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}*/