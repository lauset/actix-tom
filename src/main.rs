use actix_cors::Cors;
use actix_files::{Files, NamedFile};
use actix_settings::{ApplySettings as _, Mode, Settings};
use actix_tom::{
    app_log,
    controller::{example, masteel, weather},
    db, error, rb, CONFIG,
};
use actix_web::{
    get, guard, middleware, post, web, App, HttpResponse, HttpServer,
    Responder, Result,
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

/// 跳转首页HTML
#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../templates/index.html"))
}

/// 跳转周期图HTML
#[get("/cycle")]
async fn cycle() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../templates/charts/cycle.html"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut settings = Settings::parse_toml("./config.toml")
        .expect("Failed to parse `Settings` from config.toml");

    Settings::override_field_with_env_var(
        &mut settings.actix.hosts,
        "APPLICATION__HOSTS",
    )?;

    // fast logger 初始化
    // app_log::init_fast_logger();

    // env logger 初始化
    app_log::init_logger(&settings);

    #[cfg(debug_assertions)]
    {
        let local_ip = local_ipaddress::get().unwrap();
        log::info!("Starting Http Server :");
        log::info!(
            "Local  : http://localhost:{}",
            settings.actix.hosts[0].port
        );
        log::info!(
            "Network: http://{}:{}",
            local_ip,
            settings.actix.hosts[0].port
        );
        // log::info!("Local:    http://localhost:{}", CONFIG.SERVER.PORT);
        // log::info!("Network:  http://{}:{}", local_ip, CONFIG.SERVER.PORT);
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

    HttpServer::new({
        // clone settings into each worker thread
        let settings = settings.clone();

        log::debug!("Settings: {:?}", settings);

        move || {
            let tera =
                Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/*"))
                    .unwrap();

            App::new()
                .wrap(middleware::Compress::default())
                // 日志
                .wrap(middleware::Logger::default())
                // CORS 中间件
                .wrap(Cors::permissive())
                .app_data(rb_data.clone())
                .app_data(web::Data::new(tera))
                .app_data(web::JsonConfig::default().limit(4096))
                .app_data(web::Data::new(AppState {
                    app_name: CONFIG.APP.NAME.clone(),
                    app_version: CONFIG.APP.VERSION.clone(),
                }))
                .app_data(web::Data::new(settings.clone()))
                // register favicon
                .service(favicon)
                // html
                .service(index)
                .service(cycle)
                // config info
                .service(info)
                // example apis
                .configure(example::router)
                // weather apis
                // .configure(weather::router)
                // masteel device apis
                .configure(masteel::router)
                // static files
                .service(Files::new("/static", "static").show_files_listing())
                .route("/notfound", web::get().to(HttpResponse::NotFound))
                .service(web::scope("").wrap(error::index::error_handlers()))
        }
    })
    // .bind(("127.0.0.1", 8086))?
    // .bind(format!("{}:{}", CONFIG.SERVER.HOST, CONFIG.SERVER.PORT))?
    .apply_settings(&settings)
    .workers(2)
    .run()
    .await
}
