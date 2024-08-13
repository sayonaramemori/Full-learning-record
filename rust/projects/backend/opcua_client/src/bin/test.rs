extern crate client_test;
use chrono::Timelike;
use client_test::Example::auto_reagent::*;
use tokio;

#[tokio::main]
async fn main() {
    test().await;
    // yml_test();
}

