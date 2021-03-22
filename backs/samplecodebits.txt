/*use ferris_says::say; // import function say from ferris_says
use std::io::{stdout, BufWriter}; // import module io from std and the functions stdout, bufwriter from io
use std::fmt;*/

// extern crate env_logger;
// #[macro_use]
// extern crate log;
extern crate simple_server;
use simple_server::{Method, Server, StatusCode};

/*fn sub_function(mystr: &str) -> String {
    let my_upper_str = mystr.to_uppercase();
    let stringme: Vec<&str> = my_upper_str.split_whitespace().collect();
    println!("len is: {}", stringme.len());
    let out = String::from(format!("hell format: {}", stringme[1]));
    return out;
}

// void main
fn main() {
    let stdout = stdout(); // create a new stdout "pointer, node"
    // let message = String::from("just another message"); // create a message of type string with ... content
    // let width = message.chars().count(); // characters in message, ahh, get it, for charcount and end of string

    let mut writer = BufWriter::new(stdout.lock()); // stream socket...
    // say(message.as_bytes(), width, &mut writer).unwrap(); // walks through bytes till end of string and then unwrapped shows it as string
    // let modified_message = String::from(format!("first string, {}", "second string")); // fprintf
    // say(modified_message.as_bytes(), modified_message.chars().count(), &mut writer).unwrap(); // fancy
    let out = sub_function("some HYPERDYPER string");
    say(out.as_bytes(), out.chars().count(), &mut writer).unwrap();
    // also nice the reference to the writer, third param in ()
    // looks c++ ish
    // les fire it up
}*/

/*fn main() {
    println!("What is your name?");
    let input = read_string();
    println!("Your name is: {}", read_cleaned_string(input));
}*/

/*fn read_string() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("can not read user input");
    return read_cleaned_string(input)
}

fn read_cleaned_string(input: String) -> String {
    let out_string = input.trim();
    return String::from(out_string)
}*/

// output formatting of enum to have it convert something to string
/*impl fmt::Display for SampleEnum { // implements in rust... for enum...
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { // check mod.rs for function fingerprint, 
       match *self { // points to self reference which references back to selected enum
           SampleEnum::FirstValue => write!(f, "val1"), // write!(f, "♥"),
           SampleEnum::SecondValue => write!(f, "val2"), // write!(f, "♦"),
           SampleEnum::ThirdValue => write!(f, "val3") // write!(f, "♠"),
       }
    }
}*/

/*enum SampleEnum {
    FirstValue = 1,
    SecondValue = 2,
    ThirdValue = 3
}*/

/*fn tri_factor(my_val: SampleEnum) -> Option<String> {
    // let output_string; // js... ts...
    let output_string = format!("{} was chosen", my_val); // std::fmt::Display fun
    //match my_val {
    //    SampleEnum::FirstValue => output_string = format!("{} was chosen", "val1"),
    //    SampleEnum::SecondValue => output_string = format!("{} was chosen", "val2"),
    //    SampleEnum::ThirdValue => output_string = format!("{} was chosen", "val3")
    //}
    return Some(String::from(output_string.to_owned()));
}*/

// interface stuff, or class stuff, depends...
/*struct Car {
    name: String,
    serialno: i32,
    wheels: i8
}*/

#[allow(non_snake_case)]
struct Customers {
    customerName: String,
}

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

/*fn main() {
    // println!("{}", "lets define a car...");
    // print!("{}", "please give a name for it: ");
    // let _lname = read_string("string");
    // print!("{}", "and a serial number: ");
    // let _lserialno = read_string("i32");
    // print!("{}", "please define the number of wheels: ");
    // let _lwheels = read_string("i8");

    // println!("please choose a val between 1 and 3:");
    // let input_string = read_string(); // std::string::String
    // let my_string: Option<String>;
    // let my_string: String = String::from(input_string);

    //match input_string.as_ref() {
    //    "1" => my_string = tri_factor(SampleEnum::FirstValue).to_owned(),
    //    "2" => my_string = tri_factor(SampleEnum::SecondValue).to_owned(),
    //    "3" => my_string = tri_factor(SampleEnum::ThirdValue).to_owned(),
    //    _ => my_string = Some(format!("{}", "no enum chosen").to_owned())
    //}

    //if input_string == "1" {
    //    my_string = tri_factor(SampleEnum::FirstValue).to_owned();
    //} else if input_string == "2" {
    //    my_string = tri_factor(SampleEnum::SecondValue).to_owned();
    //} else {
    //    my_string = tri_factor(SampleEnum::ThirdValue).to_owned();
    //}

    // let my_string: Option<String> = tri_factor(SampleEnum::FirstValue).to_owned();
    // let stdout = stdout();
    // let mut writer = BufWriter::new(stdout.lock()); // stream socket...
    // say(my_string.as_bytes(), my_string.chars().count(), &mut writer).unwrap();

    // let opt: Option<String> = Some("some value".to_owned());
    // let value = my_string.as_deref().unwrap_or("default string"); // or... having something as fallback
    // say(value.as_bytes(), value.chars().count(), &mut writer).unwrap();

    // can_has_server(); // base server with routing as rust package
}*/

use sqlx::mysql::MySqlPoolOptions;
// etc.
async fn get_all_customer_names() -> Result<Vec<Customers>, sqlx::Error> {
    // struct Customers { customerName: String };
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://demo:123@127.0.0.1/classicmodels").await?;

    let rows: Vec<Customers> = sqlx::query_as!(
        Customers,
        "SELECT customerName FROM customers",
    )
    .fetch_all(&pool).await?;

    /*for row in rows {
        println!("{}", row.customerName)
    }*/

    return Ok(rows);
}

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    // _ before the var name removes the need to out it
    let _keeper: Vec<Customers> = get_all_customer_names().await.unwrap();
    Ok(can_has_server(_keeper))
}