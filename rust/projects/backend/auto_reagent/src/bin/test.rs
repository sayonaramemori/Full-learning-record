use actix_web::{cookie::time::parsing, web, App, HttpResponse, HttpServer};
use actix_cors::Cors;
use actix_redis::{RedisActor, Command, resp_array};
use actix_session::{Session, SessionMiddleware, storage::CookieSessionStore};
use actix_web::cookie::Key;
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};

extern crate AutoReagent;
use AutoReagent::handlers::{Login::*,Query::*};
use AutoReagent::AppState::RedisState;
use std::sync::Arc;



#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let url = "mysql://root:121234@kazusa.vip:3000/plc?ssl-mode=DISABLED";
    let pool = MySqlPoolOptions::new().connect(url).await.unwrap();

    let redis_client = redis::Client::open("redis://:Iloveyouxuwu121234@kazusa.vip").unwrap();
    let app_state = RedisState{
        redis_client: Arc::new(redis_client),
        redis_passwd: "Iloveyouxuwu121234".to_string(),
    };

    HttpServer::new(move || {
       let cors = Cors::default()
             .allow_any_origin()
             .allow_any_header()
             .allow_any_method()
             .supports_credentials()
            //  .allowed_methods(vec!["GET", "POST"])
            //  .allowed_headers(vec![actix_web::http::header::AUTHORIZATION, actix_web::http::header::ACCEPT])
            //  .allowed_header(actix_web::http::header::CONTENT_TYPE)
             .max_age(3600);
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(app_state.clone()))
            .wrap(cors)
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_secure(false)
                    .build()
            )
            // .default_service(web::to(|| HttpResponse::Ok()))
            .service(findlast)
            .service(login)
            .service(logout)
            .service(state)
            .service(check_privilege)
    })
    .bind("localhost:8080")?
    .run()
    .await
}
