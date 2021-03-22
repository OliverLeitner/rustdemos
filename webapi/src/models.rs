use crate::schema::*;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
//#[derive(Debug, Serialize, Deserialize, Queryable)]
#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Queryable)]
pub struct Customer {
    pub customerNumber: i32,
    pub customerName: String,
    pub contactLastName: String,
    pub contactFirstName: String,
    pub phone: String,
    pub addressLine1: String,
    pub addressLine2: Option<String>,
    pub city: String,
    pub state: Option<String>,
    pub postalCode: Option<String>,
    pub country: String,
    pub salesRepEmployeeNumber: Option<i32>,
    // pub creditLimit: f32 // cannot have any nice things like decimals
}

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
    pub user_name: String,
    pub user_password: String,
    pub user_email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Login {
    pub email: String,
    pub password: String,
    #[serde(default)]
    pub remember_me: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Register {
    pub name: String,
    pub surname: String,
    pub email: String,
    pub password: String,
}