use actix_web::{get, post, web, Responder, HttpResponse};
use crate::models::LoginInfo::LoginInfo;
use actix_session::{Session, SessionMiddleware, storage::CookieSessionStore};
use uuid::Uuid;
use actix_web::cookie::Key;

#[post("/login")]
async fn login(info: web::Json<LoginInfo>, session: Session) -> HttpResponse {
    if info.username == "admin" && info.password == "password" {
        let privilege_string = Uuid::new_v4().to_string();
        session.insert("privilege_string", privilege_string.clone()).unwrap();
        HttpResponse::Ok().json(privilege_string)
    } else {
        HttpResponse::Unauthorized().json("Invalid username or password")
    }
}

#[post("/logout")]
async fn logout(session: Session) -> HttpResponse {
    session.clear();
    HttpResponse::Ok().json("Logged out")
}

#[get("/check_privilege")]
async fn check_privilege(session: Session) -> HttpResponse {
    if let Some(privilege_string) = session.get::<String>("privilege_string").unwrap() {
        HttpResponse::Ok().json(privilege_string)
    } else {
        HttpResponse::Unauthorized().json("No privilege string found")
    }
}

