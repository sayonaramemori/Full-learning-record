extern crate client_test;
use client_test::*;

fn main() {
    let session = create_session("opc.tcp://127.0.0.1:49320");
    do_test(session);
}

