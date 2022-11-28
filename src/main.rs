use actix_files::{Files, NamedFile};
use actix_tom::{
    app_log,
    controller::{masteel, weather},
    db, error, rb, CONFIG,
};
use actix_web::{
    get, guard, middleware, post, web, App, HttpResponse, HttpServer, Responder, Result,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};
use std::{env, io};
use tera::Tera;

use rbatis::Rbatis;
use rbdc_mssql::driver::MssqlDriver;

struct AppState {
    app_name: String,
    app_version: String,
}

/// favicon handler
#[get("/favicon")]
async fn favicon() -> Result<impl Responder> {
    Ok(NamedFile::open("/static/favicon.ico")?)
}

#[get("/info")]
async fn info(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    let app_ver: &String = &&data.app_version;
    format!("Welcome to {app_name} ({app_ver})!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn home_index() -> impl Responder {
    "Home Index"
}

/// 跳转首页HTML文件
#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../templates/index.html"))
}

async fn user_detail_get(path: web::Path<(String,)>) -> HttpResponse {
    HttpResponse::Ok().body(format!("User detail: {}", path.into_inner().0))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // env logger 初始化
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // fast logger 初始化
    // app_log::init_logger();

    #[cfg(debug_assertions)]
    {
        let local_ip = local_ipaddress::get().unwrap();
        log::info!("Starting Http Server :");
        log::info!("Local:    http://localhost:{}", CONFIG.PORT);
        log::info!("Network:  http://{}:{}", local_ip, CONFIG.PORT);
    }

    let rb_data = web::Data::new(rb::init_mssql_pool());

    // let rb_data = web::Data::new(rb::init_mysql_pool());

    // let rbatis = Rbatis::new();
    // rb.init(SqliteDriver {}, "sqlite://target/sqlite.db").unwrap();
    // rb.init(MssqlDriver{},"jdbc:sqlserver://172.17.21.4:1433;User=xcsc;Password={p@ssw0rd};Database=sbgz").unwrap();

    // let sqlite_database_url =
    //     env::var("SQLITE_DATABASE_URL").expect("SQLITE_DATABASE_URL must be set");
    // log::info!("Sqlite Database Url at {}", &sqlite_database_url);

    // mssql
    // let pool = db::db_mssql::init_pool("mssql://xcsc:p@ssw0rd@172.17.21.4:1433?database=sbgz")
    //     .await
    //     .expect("Failed to create pool");

    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .app_data(rb_data.clone())
            .app_data(web::Data::new(tera))
            .app_data(web::JsonConfig::default().limit(4096))
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Tom"),
                app_version: String::from("v0.1.0"),
            }))
            // register favicon
            .service(favicon)
            .service(web::scope("/home").route("/index.html", web::get().to(home_index)))
            .service(index)
            .service(info)
            .service(echo)
            .service(
                web::resource("/user/{name}")
                    .name("user_detail")
                    .guard(guard::Header("content-type", "application/json"))
                    .route(web::get().to(user_detail_get))
                    .route(web::put().to(HttpResponse::Ok)),
            )
            // wea api
            // .configure(weather::router)
            // masteel device api
            .configure(masteel::router)
            // static files
            .service(Files::new("/static", "static").show_files_listing())
            .route("/notfound", web::get().to(HttpResponse::NotFound))
            .service(web::scope("").wrap(error::index::error_handlers()))
    })
    // .bind(("127.0.0.1", 8086))?
    .bind(format!("{}:{}", CONFIG.BIND_HOST, CONFIG.PORT))?
    .workers(2)
    .run()
    .await
}
