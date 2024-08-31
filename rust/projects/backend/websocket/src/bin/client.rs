#![feature(stmt_expr_attributes)]

use serde::{Deserialize, Serialize};
use tokio_util::sync::CancellationToken;
use tokio_tungstenite::connect_async;

use tokio_tungstenite::tungstenite::protocol::Message;
use std::{str::FromStr, sync::Arc, time::Duration};
use tokio::time::sleep;
use std::sync::mpsc::channel;

use tokio_tungstenite::MaybeTlsStream;
use tokio_tungstenite::WebSocketStream;
use tokio::net::TcpStream;
use futures::stream::{StreamExt,SplitStream,SplitSink};
use futures::SinkExt;

use AutoReagent::models::redis_data::RedisState as RedisData;
use client_test::opcua_config::data_adaptor::transferee::DataTransferee;
use client_test::opcua_config::data_adaptor::unit::Instruction::Instruction;
use client_test::opcua_config::data_adaptor::interface::transfer::InstructionInfo;
use client_test::debug_println;




#[tokio::main]
async fn main() {
    let wait_time = Duration::from_secs(5);
    loop {
        match connect_to_server().await {
            Ok(_) => { },
            Err(e) => {
                eprintln!("Fail to Connect for {e}");
                sleep(wait_time).await;
            }
        }
    }
}

async fn connect_to_server() -> Result<(), Box<dyn std::error::Error>> {
    debug_println!("Connect func called");
    let url = "ws://47.92.144.135:8080/ws"; 
    let (ws_stream, _) = connect_async(url).await?;
    let (mut write, mut read) = ws_stream.split();
    let read_handle = tokio::spawn(handle_read(read));
    let write_handle = tokio::spawn(keep_alive(write));

    if let Err(e) = tokio::try_join!(read_handle, write_handle) {
        eprintln!("Error in WebSocket communication: {:?}", e);
    }

    Ok(())
}

async fn handle_read(mut read:SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>){
        while let Some(message) = read.next().await {
            match message {
                Ok(Message::Text(text)) => {
                    let instruction = serde_json::from_str::<Instruction>(&text);
                    match instruction {
                        Ok(res) => {
                            handle_instruction(res).await
                        },
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


async fn handle_instruction(instruction: Instruction) {
    debug_println!("Receive {:?}",instruction);
    if DataTransferee::execute(&instruction).await {
        let redis_data = RedisData::new_arc();
        let status_key = format!("{}Status",instruction.target);
        redis_data.setex_retry(&status_key, instruction.value,10,5).await;
    }
}