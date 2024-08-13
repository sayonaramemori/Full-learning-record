use actix_web::{get, guard, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_cors::Cors;
use sqlx::mysql::MySqlPoolOptions;
use serde_json;
extern crate AutoReagent;
use AutoReagent::handlers::MachineButton::*;
use AutoReagent::handlers::{Login::*,Query::*};
use AutoReagent::models::redis_data::RedisState;
use std::sync::{Arc,Mutex};
use actix_web_actors::ws;
use actix::prelude::*;
use serde::{Deserialize,Serialize};
use actix::{Actor, StreamHandler};

#[derive(Message, Deserialize,Clone,Serialize)]
#[rtype(result = "()")]
struct Instruction {
    target: String,
    value: String,
}

#[get("/si")]
async fn send_instruction(
    instruction: web::Json<Instruction>,
    data: web::Data<Arc<RwLock<Vec<Addr<MyWs>>>>>,
) -> HttpResponse {
    let mut guard = data.write().unwrap();
    guard.retain(|x| x.connected());
    println!("Alive size: {}",guard.len());
    if guard.is_empty() {
        HttpResponse::InternalServerError().body("No WebSocket connection")
    }else{
        let ins = instruction.into_inner();
        for addr in guard.iter(){
            addr.do_send(ins.clone());
        }
        HttpResponse::Ok().body("Instruction sent")
    }
}
struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

//handle receive
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx : &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                //do nothing for in coming msg
                println!("Received: {}", text);
                // ctx.text(format!("Echo: {}", text));
            },
            Ok(ws::Message::Binary(binary)) => ctx.binary(binary),
            Ok(ws::Message::Ping(ping)) => ctx.ping(&ping),
            Ok(ws::Message::Pong(pong)) => ctx.pong(&pong),
            Ok(ws::Message::Close(_)) => { },
            // Err(_) => {}
            _ => {
                println!("Ji le");
            }
        }
    }
}

//handle send
impl Handler<Instruction> for MyWs {
    type Result = ();
    fn handle(&mut self, msg: Instruction, ctx: &mut Self::Context) {
        let response = format!("target: {}, Value: {}", msg.target, msg.value);
        println!("{response}");
        let res = serde_json::to_string(&msg).unwrap();
        ctx.text(res);
    }
}

#[get("/ws")]
async fn index(req: HttpRequest, stream: web::Payload, addr: web::Data<Arc<RwLock<Vec<Addr<MyWs>>>>>) -> Result<HttpResponse,Error> {
    let res = ws::WsResponseBuilder::new(MyWs, &req, stream).start_with_addr();
    let addr = addr.into_inner();
    let mut guard =  addr.write().unwrap();
    if res.is_err(){
        return Err(res.err().unwrap());
    }
    else {
        let (addr,response) = res.unwrap();
        println!("{:?}",addr);
        guard.push(addr);
        return Ok(response);
    }
}

use std::sync::RwLock;
#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let url = "mysql://root:121234@kazusa.vip:3000/plc?ssl-mode=DISABLED";
    let pool = MySqlPoolOptions::new().connect(url).await.unwrap();

    // let redis_client = redis::Client::open("redis://:Iloveyouxuwu121234@kazusa.vip").unwrap();
    let app_state = RedisState::new("Iloveyouxuwu121234", "redis://:Iloveyouxuwu121234@kazusa.vip");
    let addr: Arc<RwLock<Vec<Addr<MyWs>>>> = Arc::new(RwLock::new(vec![]));
    HttpServer::new(move || {
       let cors = Cors::default()
             .allow_any_origin()
             .allow_any_header()
             .allow_any_method()
            .allowed_origin("http://localhost:5173")
             .supports_credentials()
             .allowed_methods(vec!["GET", "POST"])
             .allowed_headers(vec![actix_web::http::header::AUTHORIZATION, actix_web::http::header::ACCEPT])
             .allowed_header(actix_web::http::header::CONTENT_TYPE)
             .max_age(3600);
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(app_state.clone()))
            .app_data(web::Data::new(addr.clone()))
            .wrap(cors)
            // .default_service(web::to(|| HttpResponse::Ok()))
            .service(findlast)
            .service(findlast_vice)
            .service(login)
            .service(logout)
            .service(turbine_state)
            .service(check_privilege)
            .service(history)
            .service(start)
            .service(stop)
            .service(pump_status)
            .service(set_point)
            .service(index)
            .service(send_instruction)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
