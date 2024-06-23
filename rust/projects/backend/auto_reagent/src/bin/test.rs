use actix_web::{App, HttpServer};
use actix_cors::Cors;
// use std::env;

extern crate AutoReagent;
use AutoReagent::handlers::{Login::*,Query::*};

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    //let ip:String = env::var("IP").unwrap();
    HttpServer::new(|| {
       let cors = Cors::default()
             .allow_any_origin()
             .allowed_methods(vec!["GET", "POST"])
             .allowed_headers(vec![actix_web::http::header::AUTHORIZATION, actix_web::http::header::ACCEPT])
             .allowed_header(actix_web::http::header::CONTENT_TYPE)
             .max_age(3600);
        App::new()
            .wrap(cors)
            .service(findlast)
            .service(greet)
            .service(login)
            .service(logout)
            .service(check_privilege)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
