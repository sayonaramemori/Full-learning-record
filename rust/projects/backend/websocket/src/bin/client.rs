use serde::Deserialize;
use tokio_util::sync::CancellationToken;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use futures::stream::StreamExt;
use futures::SinkExt;
use std::time::Duration;
use tokio::time::sleep;
use std::sync::mpsc::channel;



#[derive(Deserialize,Debug)]
struct Instruction {
    action: String,
    value: i32,
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
    let read_task = tokio::spawn(async move {
        while let Some(message) = read.next().await {
            match message {
                Ok(Message::Text(text)) => {
                    let instruction = serde_json::from_str(&text);
                    match instruction {
                        Ok(res) => handle_instruction(res).await,
                        _ => println!("{text}"),
                    }
                    
                }
                _ => println!("Error for message"),
            }
        }
        cancel_token_clone.cancel();
    });

    // 处理发送的消息（此处可以添加发送逻辑）
    let send_task = tokio::spawn(async move {
        // 示例：发送一个初始化消息
        while let Err(e) = write.send(Message::Text("Client connected".into())).await {
            println!("Connect failed");
        }
    });

    tokio::try_join!(read_task, send_task)?;

    Ok(())
}

async fn handle_instruction(instruction: Instruction) {
    // 创建 OPC UA 客户端
    println!("Instruction is {:?}",instruction);
}