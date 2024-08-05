use std::collections::HashMap;
use tokio::net::TcpStream;
use std::net::SocketAddr;
use std::sync::{Arc,Mutex};

use futures::channel::mpsc::{self,UnboundedSender};
use futures::{future, StreamExt, TryFutureExt, TryStreamExt};
use tokio_tungstenite::tungstenite::Message;
use tokio::net::TcpListener;

const SERVER_ADDR :&str = "localhost:8080";

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind(SERVER_ADDR).await.unwrap();
    println!("start");
    while let Ok((stream,addr)) = listener.accept().await {
        tokio::spawn(handler(stream, addr));
    }
}

async fn handler(stream:TcpStream,addr: SocketAddr){
    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Err when handshaking");
    let (writer,reader) = ws_stream.split();
    let receive_msg = reader.try_for_each(|msg|{
        match msg.clone() {
            Message::Text(msg) => println!("{addr}, with {msg}"),
            Message::Close(_) => println!("Closed"),
            _ => println!("Not support"),
        }
        future::ok(())
    });
}