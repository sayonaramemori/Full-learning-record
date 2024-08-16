#![feature(stmt_expr_attributes)]

use serde::{Deserialize, Serialize};
use tokio_util::sync::CancellationToken;
use tokio_tungstenite::connect_async;

use tokio_tungstenite::tungstenite::protocol::Message;
use std::{str::FromStr, sync::Arc, time::Duration};
use tokio::time::sleep;
use client_test::{debug_println, store::middleware::DataStore, Example::auto_reagent::{create_data_store, transfer_data_to_plc}};
use std::sync::mpsc::channel;

use tokio_tungstenite::MaybeTlsStream;
use tokio_tungstenite::WebSocketStream;
use tokio::net::TcpStream;
use futures::stream::{StreamExt,SplitStream,SplitSink};
use futures::SinkExt;


#[derive(Deserialize,Debug)]
struct Instruction {
    target: String,
    value: String,
}

#[tokio::main]
async fn main() {
    let wait_time = Duration::from_secs(5);
    loop {
        match connect_to_server().await {
            Ok(_) => {
                debug_println!("Connected");
            }
            Err(e) => {
                eprintln!("Fail to Connect for {e}");
                sleep(wait_time).await;
            }
        }
    }
}

async fn connect_to_server() -> Result<(), Box<dyn std::error::Error>> {
    debug_println!("Connect func called");
    let url = "ws://47.92.144.135:8080/ws"; // 后端 WebSocket 服务器地址
    let (ws_stream, _) = connect_async(url).await?;
    let (mut write, mut read) = ws_stream.split();
    tokio::select!{
        _ = handle_read(read) => { },
        _ = keep_alive(write) => {},
    };
    Ok(())
}

async fn handle_read(mut read:SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>){
        let ds = create_data_store(true, false, true, false).await;
        while let Some(message) = read.next().await {
            match message {
                Ok(Message::Text(text)) => {
                    let instruction = serde_json::from_str(&text);
                    match instruction {
                        Ok(res) => handle_instruction(ds.clone(),res).await,
                        Err(_) => debug_println!("Not a instruction"),
                    }
                },
                Ok(Message::Ping(_)) => {},
                Ok(Message::Pong(_)) => {},
                Ok(Message::Frame(_)) => { debug_println!("Frame gained");},
                Ok(Message::Binary(_)) => { debug_println!("Binary gained");},
                Ok(Message::Close(_)) => { debug_println!("Close gained");},
                Err(e) => eprintln!("Error message for {}",e),
            };
        }
        debug_println!("receive-task over");
}

async fn keep_alive(mut write:SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>){
    while let Ok(_) = write.send(Message::Ping(Vec::new())).await { sleep(std::time::Duration::from_secs(15)).await; }
    debug_println!("send task over");
}

async fn handle_instruction(ds:Arc<DataStore>,instruction: Instruction) {
    debug_println!("Receive {:?}",instruction);
    transfer_data_to_plc(ds,instruction.target, instruction.value).await;
    debug_println!("Write operation over");
}