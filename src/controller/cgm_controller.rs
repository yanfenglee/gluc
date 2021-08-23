use actix_web::{Responder, web, get, post};
//use chrono::NaiveDateTime;
//use rbatis::core::value::DateTimeNow;
use serde::Deserialize;

use crate::domain::vo::RespVO;
use crate::service::CGM_SERVICE;
use crate::domain::dto::BgDTO;
use rbatis::core::Error;
use crate::middleware::auth_user::AuthUser;
use crate::middleware::auth;

/// config route service
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/v1")
        .wrap(auth::UserAuth)
        .service(receive_bg)
        .service(get_bg)
        .service(device_status)
        .service(list_device_status)
        .service(get_treatments)
    );
}


/// receive bg
#[post("/entries")]
pub async fn receive_bg(user: Option<AuthUser>, arg: web::Json<Vec<BgDTO>>) -> impl Responder {
    log::info!("receive bg: {:?}", arg);

    let data = CGM_SERVICE.add(&arg, user.unwrap().user_id).await;
    RespVO::from_result(&data).resp()
}


#[derive(Debug, Deserialize)]
pub struct Info {
    count: i64,
    rr: i64,
}

#[get("/entries.json")]
pub async fn get_bg(user: Option<AuthUser>, info: web::Query<Info>) -> impl Responder {
    log::info!("query entries {:?}, {:?}", user, info);

    let data = CGM_SERVICE.list(info.rr, info.count, user.unwrap().user_id).await;
    RespVO::from_result(&data).resp()
}

#[post("/devicestatus")]
pub async fn device_status(user: Option<AuthUser>, arg: web::Json<Vec<BgDTO>>) -> impl Responder {
    log::info!("receive bg: {:?}", arg);

    let data = CGM_SERVICE.add(&arg, user.unwrap().user_id).await;
    RespVO::from_result(&data).resp()
}

#[get("/devicestatus.json")]
pub async fn list_device_status(user: Option<AuthUser>, info: web::Query<Info>) -> impl Responder {
    log::info!("query entries {:?}", info);

    let data = CGM_SERVICE.list(info.rr, info.count, user.unwrap().user_id).await;
    RespVO::from_result(&data).resp()
}

#[get("/treatments")]
pub async fn get_treatments() -> impl Responder {
    let data: Result<Vec<i32>, Error> = Ok(vec![]);
    RespVO::from_result(&data).resp()
}

