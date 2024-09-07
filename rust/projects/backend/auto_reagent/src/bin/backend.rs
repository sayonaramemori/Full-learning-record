use actix_web::{get, guard, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_cors::Cors;
use chrono::format::format;
use sqlx::{mysql::MySqlPoolOptions,MySqlPool};
extern crate AutoReagent;
use AutoReagent::handlers::MachineButton::*;
use AutoReagent::handlers::{Login::*,Query::*};
use AutoReagent::models::{redis_data::RedisState,sqlx_manager::SqlxManager};
use actix::prelude::Addr;
use std::sync::{RwLock,Arc};
use AutoReagent::websocket::myws::{Instruction,MyWs,*};
use std::collections::HashMap;


#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let db_names = ["flux","fluxVice","plc"];
    let mut sqlx_state = SqlxManager::new();
    for name in db_names {
        let url = format!("mysql://root:121234@ayanamyrei.com:3000/{name}?ssl-mode=DISABLED");
        sqlx_state.add_database(name, &url).await;
    }
    let app_state = RedisState::new("Iloveyouxuwu121234", "redis://:Iloveyouxuwu121234@ayanamyrei.com");
    let addr: Arc<RwLock<Vec<Addr<MyWs>>>> = Arc::new(RwLock::new(vec![]));
    HttpServer::new(move || {
       let cors = Cors::default()
            //  .allow_any_origin()
            //  .allow_any_header()
            //  .allow_any_method()
             .allowed_origin("http://localhost:5173")
             .allowed_origin("http://47.92.144.135")
             .supports_credentials()
             .allowed_methods(vec!["GET", "POST"])
             .allowed_headers(vec![actix_web::http::header::AUTHORIZATION, actix_web::http::header::ACCEPT,])
             .allowed_header(actix_web::http::header::CONTENT_TYPE)
             .allowed_header("token")
             .expose_headers(vec!["token"])
             .max_age(3600);
        App::new()
            .app_data(web::Data::new(sqlx_state.clone()))
            .app_data(web::Data::new(app_state.clone()))
            .app_data(web::Data::new(addr.clone()))
            .wrap(cors)
            .service(findlast)
            .service(findlast_vice)
            .service(login)
            .service(logout)
            .service(turbine_state)
            .service(check_privilege)
            .service(main_history)
            .service(vice_history)
            .service(start_operation)
            .service(stop_operation)
            .service(pump_status)
            .service(set_point)
            .service(websocket_index)
            // .service(send_instruction)
            // .default_service(web::to(|| HttpResponse::Ok()))
    })
    .bind("0.0.0.0:8080")?
    // .bind("localhost:8080")?
    .run()
    .await
}
