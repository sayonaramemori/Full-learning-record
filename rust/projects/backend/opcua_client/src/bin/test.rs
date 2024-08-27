extern crate client_test;
use chrono::Timelike;
use client_test::example::auto_reagent::*;
use tokio;

#[tokio::main]
async fn main() {
    match do_record().await {
        Ok(_) => {},
        Err(e) => {
            println!("Stop for {e}");
        }
    }
}

