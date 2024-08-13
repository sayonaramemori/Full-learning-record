use actix_web::{get, guard, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_cors::Cors;
use sqlx::mysql::MySqlPoolOptions;

extern crate AutoReagent;
use AutoReagent::handlers::MachineButton::*;
use AutoReagent::handlers::{Login::*,Query::*};
use AutoReagent::models::redis_data::RedisState;
use std::sync::{Arc,Mutex};
use actix_web_actors::ws;
use actix::prelude::*;
use serde::Deserialize;
use actix::{Actor, StreamHandler};

#[derive(Message, Deserialize)]
#[rtype(result = "()")]
struct Instruction {
    action: String,
    value: i32,
}

#[get("/si")]
async fn send_instruction(
    instruction: web::Json<Instruction>,
    data: web::Data<Arc<RwLock<Option<Addr<MyWs>>>>>,
) -> HttpResponse {
    let guard = data.write();
    let guard = guard.as_ref().unwrap().as_ref();
    match guard {
       Some(addr) => {
            addr.do_send(instruction.into_inner());
            HttpResponse::Ok().body("Instruction sent")
       },
       None => {
        HttpResponse::InternalServerError().body("No WebSocket connection")
       }
    }
}
struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx : &mut Self::Context) {
        println!("Start handle stream");
        match msg {
            Ok(ws::Message::Text(text)) => {
                println!("Received: {}", text);
                ctx.text(format!("Echo: {}", text));
                ctx.text(text)
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

impl Handler<Instruction> for MyWs {
    type Result = ();
    fn handle(&mut self, msg: Instruction, ctx: &mut Self::Context) {
        let response = format!("Action: {}, Value: {}", msg.action, msg.value);
        ctx.text(response);
    }
}

#[get("/ws")]
async fn index(req: HttpRequest, stream: web::Payload, addr: web::Data<Arc<RwLock<Option<Addr<MyWs>>>>>) -> Result<HttpResponse,Error> {
    let res = ws::WsResponseBuilder::new(MyWs, &req, stream).start_with_addr();
    let addr = addr.into_inner();
    let mut guard =  addr.write().unwrap();
    if res.is_err(){
        return Err(res.err().unwrap());
    }
    else {
        let (addr,response) = res.unwrap();
        println!("{:?}",addr);
        *guard = Some(addr);
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
    let addr: Arc<RwLock<Option<Addr<MyWs>>>> = Arc::new(RwLock::new(None));
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
    .bind("localhost:8080")?
    .run()
    .await
}
