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
use std::sync::RwLock;
use AutoReagent::websocket::myws::{Instruction,MyWs,*};


#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let url = "mysql://root:121234@kazusa.vip:3000/plc?ssl-mode=DISABLED";
    let pool = MySqlPoolOptions::new().connect(url).await.unwrap();
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
            .service(start_operation)
            .service(stop_operation)
            .service(pump_status)
            .service(set_point)
            .service(websocket_index)
<<<<<<< HEAD
            // .service(send_instruction)
=======
            .service(send_instruction)
>>>>>>> ea08cdaccdf1ff0344eeaa9771e6e37dc06f0b50
    })
    .bind("0.0.0.0:8080")?
    // .bind("localhost:8080")?
    .run()
    .await
}
