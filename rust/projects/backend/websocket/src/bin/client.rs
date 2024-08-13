use serde::Deserialize;
use tokio_util::sync::CancellationToken;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use futures::stream::StreamExt;
use futures::SinkExt;
use std::{sync::Arc, time::Duration};
use tokio::time::sleep;
use client_test::{store::app::DataStore, Example::auto_reagent::{create_data_store, test, transfer_data_to_plc}};
use std::sync::mpsc::channel;



#[derive(Deserialize,Debug)]
struct Instruction {
    target: String,
    value: String,
}

#[tokio::main]
async fn main() {
    let mut retry_attempts = 0;
    let max_retry_attempts = 10;

    loop {
        match connect_to_server().await {
            Ok(_) => {
                println!("Connected");
                retry_attempts = 0; // 重置重试计数器
            }
            Err(e) => {
                println!("Fail to Connect");
                retry_attempts += 1;
                if retry_attempts >= max_retry_attempts {
                    break;
                }
                let wait_time = Duration::from_secs(2_u64.pow(retry_attempts));
                sleep(wait_time).await;
            }
        }
    }
}

async fn connect_to_server() -> Result<(), Box<dyn std::error::Error>> {
    println!("Connect func called");
    let url = "ws://47.92.144.135:8080/ws"; // 后端 WebSocket 服务器地址
    let (ws_stream, _) = connect_async(url).await?;

    let (mut write, mut read) = ws_stream.split();

    let cancellation_token = CancellationToken::new();
    let cancel_token_clone = cancellation_token.clone();
    // 启动一个任务处理从 WebSocket 接收的消息
    let plc_record = tokio::spawn(test());
    let read_task = tokio::spawn(async move {
        let ds = create_data_store(true, false, true, true).await;
        while let Some(message) = read.next().await {
            match message {
                Ok(Message::Text(text)) => {
                    let instruction = serde_json::from_str(&text);
                    match instruction {
                        Ok(res) => handle_instruction(ds.clone(),res).await,
                        _ => println!("sb b {text}"),
                    }
                    
                }
                _ => println!("Error for message"),
            }
        }
        cancel_token_clone.cancel();
    });

    let send_task = tokio::spawn(async move {
        while let Ok(_) = write.send(Message::Text("Ping".into())).await {
            sleep(std::time::Duration::from_secs(5)).await;
        }
    });

    tokio::try_join!(read_task, send_task, plc_record)?;

    Ok(())
}

async fn handle_instruction(ds:Arc<DataStore>,instruction: Instruction) {
    //My important 
    println!("Instruction is {:?}",instruction);
    transfer_data_to_plc::<f64>(ds,instruction.target, instruction.value).await;
    println!("Handler over");
}