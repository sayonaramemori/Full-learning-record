extern crate client_test;
use client_test::example::auto_reagent::*;
use tokio;

#[tokio::main]
async fn main() {
    match do_record().await {
        Ok(_) => {},
        Err(_e) => {}
    }
}

