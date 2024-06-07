use actix_web::{App, HttpServer};
use actix_cors::Cors;
// use std::env;

extern crate AutoReagent;
use AutoReagent::handlers::*;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    //let ip:String = env::var("IP").unwrap();
    HttpServer::new(|| {
       let cors = Cors::default()
             .allowed_origin("http://localhost:5173")
             .allowed_methods(vec!["GET", "POST"])
             .max_age(3600);
        App::new()
            .wrap(cors)
            .service(findlast)
            .service(greet)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
