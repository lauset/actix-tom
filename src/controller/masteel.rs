use actix_web::web::{self, ServiceConfig};

pub mod device;
pub mod screen;

// routes
pub fn router(config: &mut ServiceConfig) {
    config.service(
        web::scope("/api/masteel").service(
            web::scope("/device")
                .service(device::get_device_list)
                .service(device::get_device_report_list)
                .service(device::get_device_area)
                .service(device::post_device_test)
                .service(device::get_device_test),
        ),
    );
}
