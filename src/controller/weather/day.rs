use crate::model::ResponseData;
use actix_web::{error, get, web, Error, HttpResponse, Responder};
use rbatis::{executor::RbatisRef, Rbatis};
use std::borrow::BorrowMut;

// use crate::model::{Claim, RealWorldToken, ResponseData, UpdateUser, UpdateUserPayload};
use crate::service::weather::day;
// use crate::util::error::CustomError::InternalError;

#[get("/{num}")]
pub async fn get_day_num(
    data: web::Data<Rbatis>,
    path: web::Path<u32>,
) -> Result<impl Responder, actix_web::Error> {
    let num = path.into_inner();
    warn!("num: {}", &num);
    let rbatis = data.get_rbatis();
    let data = day::select_day_by_num(rbatis, num)
        .await
        .map_err(error::ErrorInternalServerError)?;
    Ok(ResponseData::new("data", data))
}

#[get("")]
pub async fn day_index(wd: web::Data<Rbatis>) -> impl Responder {
    warn!("api wea day");
    HttpResponse::Ok().body("Hey there")
}
