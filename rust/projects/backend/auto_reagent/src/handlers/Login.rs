use actix_web::{get, post, web, Responder, HttpResponse};
use mysql::binlog::row;
use serde::{Deserialize, Serialize};
use sqlx::{MySql, MySqlPool};
use crate::models::{LoginInfo::{LoginInfo,Identity},TurbineState::TurbineState};
use actix_session::Session;
use uuid::Uuid;


#[post("/login")]
async fn login(info: web::Json<LoginInfo>, session: Session, pool:web::Data<MySqlPool>) -> HttpResponse {
    let user_info = sqlx::query_as::<MySql,LoginInfo>("select username, password from admin where username=? and password=?")
        .bind(&info.username)
        .bind(&info.password)
        .fetch_one(pool.get_ref())
        .await;
    match user_info {
        Ok(_) => {
            let privilege= Uuid::new_v4().to_string();
            session.insert("privilege", privilege.clone()).unwrap();
            session.insert("username", info.username.clone()).unwrap();
            HttpResponse::Ok().json(privilege)
        },
        _ => {
            HttpResponse::Unauthorized().json("Invalid username or password")
        },
    }
}

#[post("/logout")]
async fn logout(session: Session) -> HttpResponse {
    session.clear();
    HttpResponse::Ok().json("Logged out")
}

#[get("/check_privilege")]
async fn check_privilege(session: Session) -> HttpResponse {
    if let (Some(privilege),Some(username)) = (session.get::<String>("privilege").unwrap(),session.get::<String>("username").unwrap()) {
        HttpResponse::Ok().json(Identity {username,privilege})
    } else {
        HttpResponse::Unauthorized().json("No privilege string found")
    }
}

#[get("/state")]
async fn state(session: Session,pool: web::Data<MySqlPool>) -> HttpResponse {
    if let Some(_)= session.get::<String>("privilege").unwrap() {
        let rows = sqlx::query_as::<MySql,TurbineState>("select * from turbineState").fetch_all(pool.get_ref()).await.unwrap();
        HttpResponse::Ok().json(rows)
    } else {
        HttpResponse::Unauthorized().json("No privilege string found")
    }
}






