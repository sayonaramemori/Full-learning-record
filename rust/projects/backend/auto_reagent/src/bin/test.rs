use actix_web::{http, App, HttpServer};
use actix_cors::Cors;
use std::env;

extern crate AutoReagent;
use AutoReagent::handlers::*;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let ip:String = env::var("IP").unwrap();
    HttpServer::new(|| {
//       let cors = Cors::default()
//             .allowed_origin("http://211.70.208.202")
//             .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
//             .allowed_header(http::header::CONTENT_TYPE)
//             .allowed_methods(vec!["GET", "POST"])
//             .max_age(3600);
        App::new()
            //.wrap(cors)
            .service(findlast)
            .service(greet)
    })
    .bind((ip, 8080))?
    .run()
    .await
}
