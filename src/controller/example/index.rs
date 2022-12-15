use actix_web::{get, post, web, HttpResponse, Responder};

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// pub async fn echo_str() -> impl Responder {
//     "Some String"
// }

pub async fn user_detail_get(path: web::Path<(String,)>) -> HttpResponse {
    HttpResponse::Ok().body(format!("User detail: {}", path.into_inner().0))
}
