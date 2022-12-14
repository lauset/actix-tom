use actix_web::{
    http::header::ContentType, HttpRequest, HttpResponse, Responder,
};
use fancy_regex::Regex;

pub mod response;
pub mod tbl_warning;
pub mod wea_sys_user;

pub use response::*;
pub use tbl_warning::*;
pub use wea_sys_user::*;

// 统一响应结构体
#[derive(Debug, serde::Serialize)]
pub struct ResponseData {
    pub body: String,
}

impl ResponseData {
    pub fn new<T>(property_name: &str, data: T) -> Self
    where
        T: serde::Serialize,
    {
        Self {
            body: json!({ property_name: data }).to_string(),
        }
    }

    pub fn same<T>(data: T) -> Self
    where
        T: serde::Serialize,
    {
        let type_name = std::any::type_name::<T>();
        let list: Vec<&str> = type_name.rsplitn(2, "::").collect();
        let type_name = list[0].replace('>', "").to_lowercase();
        let regex = Regex::new(r"^(?<=<).+?(?=>)$").unwrap();
        let result = regex.find(&type_name);
        let type_name = match result {
            Err(_) => &type_name,
            Ok(option) => match option {
                None => &type_name,
                Some(m) => m.as_str(),
            },
        };
        Self::new(type_name, data)
    }
}

// 为返回体实现 actix Responder
impl Responder for ResponseData {
    type Body = actix_http::body::BoxBody;

    fn respond_to(self, _request: &HttpRequest) -> HttpResponse<Self::Body> {
        // Create HttpResponse and set Content Type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(self.body)
    }
}
