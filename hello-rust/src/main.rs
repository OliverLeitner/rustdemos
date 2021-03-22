// extern crate env_logger;
// #[macro_use]
// extern crate log;
extern crate async_trait;
extern crate simple_server;
use simple_server::{Method, Server, StatusCode};

trait MyServer {
    fn can_has_server(rows: Vec<Customers>);
}

struct LocalServer;

impl MyServer for LocalServer {
    fn can_has_server(rows: Vec<Customers>) {
        let host = "0.0.0.0";
        let port = "7878";

        let mut display_rows = String::from("<h1>Our customers!</h1><ul>").to_owned();

        for row in rows {
            let mut new_customer = String::new().to_owned();
            new_customer.push_str("<li>");
            new_customer.push_str(&row.customerName);
            new_customer.push_str("</li>");
            display_rows.push_str(&new_customer);
        }

        display_rows.push_str(&"</ul>");

        let server = Server::new(move |request, mut response| {
            // info!("Request received. {} {}", request.method(), request.uri());

            match (request.method(), request.uri().path()) {
                (&Method::GET, "/hello") => {
                    Ok(response.body("<h1>Hi!</h1><p>Hello Rust!</p>".as_bytes().to_vec())?)
                }
                (&Method::GET, "/customers") => {
                    Ok(response.body(display_rows.as_bytes().to_vec())?)
                }
                (_, _) => {
                    response.status(StatusCode::NOT_FOUND);
                    Ok(response.body("<h1>404</h1><p>Not found!<p>".as_bytes().to_vec())?)
                }
            }
        });

        server.listen(host, port);
    }
}
// extern crate env_logger;
// #[macro_use]
// extern crate log;
// extern crate simple_server;
// use simple_server::{Method, Server, StatusCode};

#[allow(non_snake_case)]
struct Customers {
    customerName: String,
}

use async_trait::async_trait;
use sqlx::mysql::MySqlPoolOptions;

#[async_trait]
trait MyDatabase {
    async fn get_all_customer_names() -> Result<Vec<Customers>, sqlx::Error>;
}

struct LocalDatabase;

#[async_trait]
impl MyDatabase for LocalDatabase {
    async fn get_all_customer_names() -> Result<Vec<Customers>, sqlx::Error> {
        // struct Customers { customerName: String };
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect("mysql://demo:123@127.0.0.1/classicmodels")
            .await?;

        let rows: Vec<Customers> =
            sqlx::query_as!(Customers, "SELECT customerName FROM customers",)
                .fetch_all(&pool)
                .await?;

        /*for row in rows {
            println!("{}", row.customerName)
        }*/

        return Ok(rows);
    }
}

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    // _ before the var name removes the need to out it
    let _keeper: Vec<Customers> = LocalDatabase::get_all_customer_names().await.unwrap();
    Ok(LocalServer::can_has_server(_keeper))
}
