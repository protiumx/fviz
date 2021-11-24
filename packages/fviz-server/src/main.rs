use std::env;

extern crate dotenv;

fn main() {
    dotenv::dotenv().expect("Failed to read .env file");
    println!("Host: {}", env::var("WS_HOST").unwrap());
}
