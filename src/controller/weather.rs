use actix_web::web::{self, ServiceConfig};
// use crate::controller::weather::day;

pub mod day;

// routes
pub fn router(config: &mut ServiceConfig) {
    config.service(
        web::scope("/api/wea").service(
            web::scope("/day")
                .service(day::get_day_num)
                .service(day::day_index),
        ),
    );
}
