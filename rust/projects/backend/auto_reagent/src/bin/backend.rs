use actix_web::{get, guard, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_cors::Cors;
use sqlx::mysql::MySqlPoolOptions;
extern crate AutoReagent;
use sqlx::MySqlPool;
use AutoReagent::handlers::MachineButton::*;
use AutoReagent::handlers::{Login::*,Query::*};
use AutoReagent::models::redis_data::RedisState;
use actix::prelude::Addr;
use std::sync::{RwLock,Arc};
use AutoReagent::websocket::myws::{Instruction,MyWs,*};
use std::collections::HashMap;


#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let db_url1 = "mysql://root:121234@ayanamyrei.com:3000/flux?ssl-mode=DISABLED";
    let db_url2 = "mysql://root:121234@ayanamyrei.com:3000/fluxVice?ssl-mode=DISABLED";
    let db_url3 = "mysql://root:121234@ayanamyrei.com:3000/plc?ssl-mode=DISABLED";
    let pool_1 = MySqlPoolOptions::new().connect(db_url1).await.unwrap();
    let pool_2 = MySqlPoolOptions::new().connect(db_url2).await.unwrap();
    let pool_3 = MySqlPoolOptions::new().connect(db_url3).await.unwrap();
    let mut db_pools: HashMap<String, MySqlPool> = HashMap::new();
    db_pools.insert("flux".to_string(), pool_1);
    db_pools.insert("fluxVice".to_string(), pool_2);
    db_pools.insert("plc".to_string(), pool_3);
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
            .app_data(web::Data::new(db_pools.clone()))
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
            .service(main_history)
            .service(vice_history)
            .service(start_operation)
            .service(stop_operation)
            .service(pump_status)
            .service(set_point)
            .service(websocket_index)
            // .service(send_instruction)
    })
    .bind("0.0.0.0:8080")?
    // .bind("localhost:8080")?
    .run()
    .await
}
