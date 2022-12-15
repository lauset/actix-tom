use crate::constants;
use crate::error::service::ServiceError;
use crate::model::{
    response::ResponseBody, ResponseData, TblWarning, TblWarningQuery,
};
use actix_http::StatusCode;
use actix_web::http::header::ContentType;
use actix_web::{
    error, get, web, Error, HttpRequest, HttpResponse, Responder,
    Result as WebResult,
};
use async_stream::stream;
use fast_log::print;
use rbatis::{executor::RbatisRef, rbdc::datetime::FastDateTime, Rbatis};
use std::convert::Infallible;
use std::fmt::Debug;
use std::{borrow::BorrowMut, str::FromStr};

// use crate::util::error::CustomError::InternalError;
use crate::service::masteel::device as device_service;

/// 访问路径: /api/masteel/device/list
/// 请求参数: TblWarningQuery
#[get("list")]
pub async fn get_device_list(
    query: web::Query<TblWarningQuery>,
    data: web::Data<Rbatis>,
) -> Result<impl Responder, Error> {
    let default_dt = FastDateTime(
        fastdate::DateTime::from_str("1970-01-01 00:00:00").unwrap(),
    );
    let q = query.into_inner();
    let start_time = q.start_time.unwrap_or(default_dt.clone());
    let end_time = q.end_time.unwrap_or(default_dt.clone());
    let eq_area = q.eq_area.unwrap_or("".to_string());
    let eq_name = q.eq_name.unwrap_or("".to_string());
    let status = q.status.unwrap_or("".to_string());
    let level: u8 = q.level.unwrap_or(3);
    let time_min = q.time_min.unwrap_or(1 * 60);
    let order_by = q.order_by.unwrap_or("".to_string());
    warn!(
        "   
        ----------
        masteel device list query:
        ---------- 
        eq name: {:?}, 
        time: {:?} - {:?}
        status: {}
        level: {}
        fd_min: {}
        order_by: {}",
        &eq_name, &start_time, &end_time, &status, &level, &time_min, &order_by
    );
    let mut rbatis = data.get_rbatis();
    let data = device_service::select_list(
        &mut rbatis,
        &start_time,
        &end_time,
        eq_area.as_str(),
        eq_name.as_str(),
        status.as_str(),
        level,
        time_min,
        order_by.as_str(),
    )
    .await
    .map_err(error::ErrorInternalServerError)?;
    Ok(ResponseData::new("data", data))
}

/// 访问路径: /api/masteel/device/report/list
/// 请求参数: TblWarningQuery
#[get("report/list")]
pub async fn get_device_report_list(
    query: web::Query<TblWarningQuery>,
    data: web::Data<Rbatis>,
) -> WebResult<HttpResponse> {
    let default_dt = FastDateTime(
        fastdate::DateTime::from_str("1970-01-01 00:00:00").unwrap(),
    );
    let q = query.into_inner();
    let start_time = q.start_time.unwrap_or(default_dt.clone());
    let end_time = q.end_time.unwrap_or(default_dt.clone());
    let eq_area = q.eq_area.unwrap_or("".to_string());
    let eq_name = q.eq_name.unwrap_or("".to_string());
    let status = q.status.unwrap_or("".to_string());
    let level: u8 = q.level.unwrap_or(3);
    let time_min = q.time_min.unwrap_or(1 * 60);
    let order_by = q.order_by.unwrap_or("".to_string());
    warn!(
        "   
        ----------
        masteel device report list query:
        ---------- 
        eq area: {:?}
        eq name: {:?},
        time: {:?} - {:?}
        status: {}
        level: {}
        fd_min: {}
        order_by: {}",
        &eq_area,
        &eq_name,
        &start_time,
        &end_time,
        &status,
        &level,
        &time_min,
        &order_by
    );
    let mut rbatis = data.get_rbatis();
    let data_result = device_service::select_report_list(
        &mut rbatis,
        &start_time,
        &end_time,
        eq_area.as_str(),
        eq_name.as_str(),
        status.as_str(),
        level,
        time_min,
        order_by.as_str(),
    )
    .await;
    // .map_err(error::ErrorInternalServerError)?;
    // Ok(ResponseData::new("data", data))

    match data_result {
        Ok(data) => Ok(HttpResponse::Ok()
            .json(ResponseBody::new(constants::MESSAGE_OK, data))),
        Err(err) => {
            Ok(ServiceError::new(StatusCode::BAD_REQUEST, err.to_string())
                .response())
        }
    }
}

#[get("area/{name}")]
pub async fn get_device_area(
    path: web::Path<String>,
    data: web::Data<Rbatis>,
) -> Result<impl Responder, Error> {
    let area_name = path.into_inner();
    warn!(
        "   
        ----------
        masteel device area path:
        ---------- 
        area name: {:?}",
        area_name,
    );
    let mut rbatis = data.get_rbatis();
    let rbatis = rbatis.borrow_mut();
    let data = TblWarning::select_device_by_eqarea(rbatis, &area_name)
        .await
        .map_err(error::ErrorInternalServerError)?;
    Ok(ResponseData::new("data", data))
}

#[post("/posttest")]
async fn post_device_test(
    q: web::Json<TblWarningQuery>,
    req: HttpRequest,
) -> HttpResponse {
    println!("model: {q:?}");
    HttpResponse::Ok().json(q.0)
}

#[get("/gettest")]
async fn get_device_test(
    q: web::Query<TblWarningQuery>,
    req: HttpRequest,
) -> HttpResponse {
    let qi = q.into_inner();
    info!("{req:?}");
    info!("query: {qi:?}");
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .streaming(stream! {
            yield Ok::<_, Infallible>(web::Bytes::from("Hello "));
            yield Ok::<_, Infallible>(web::Bytes::from("World "));
            yield Ok::<_, Infallible>(web::Bytes::from("!"));
        })
}
