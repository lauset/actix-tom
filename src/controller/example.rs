use actix_web::web::{self, ServiceConfig};
use actix_web::{guard, HttpResponse};

mod index;

use self::index::{echo, user_detail_get};

// routes
pub fn router(config: &mut ServiceConfig) {
    config.service(
        web::scope("/example")
            .service(
                web::resource("/user/{name}")
                    .name("user_detail")
                    .guard(guard::Header("content-type", "application/json"))
                    .route(web::get().to(user_detail_get))
                    .route(web::put().to(HttpResponse::Ok)),
            )
            .service(echo),
    );
}
