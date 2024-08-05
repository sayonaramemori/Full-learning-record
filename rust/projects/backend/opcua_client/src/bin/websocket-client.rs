use std::net::TcpStream;
use tungstenite::connect;
use tungstenite::protocol::Message;

fn main() {
    let (mut socket, response) =
        connect("ws://localhost:8080/ws").expect("Can't connect");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());

    loop {
        let msg = socket.read_message().expect("Error reading message");
        match msg {
            Message::Text(txt) => {
                println!("Received instruction: {}", txt);
                // 在这里处理接收到的指令，例如通过 OPC UA 协议与 PLC 通信
            }
            _ => (),
        }
    }
}