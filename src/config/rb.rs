use crate::config::CONFIG;
use rbatis::Rbatis;
use rbdc_mssql::driver::MssqlDriver;
use rbdc_mysql::driver::MysqlDriver;

pub fn init_mysql_pool() -> Rbatis {
    #[cfg(debug_assertions)]
    let mysql_url = {
        debug!("rbatis pool init ({})...", CONFIG.DATABASE.MYSQL_URL);
        CONFIG.DATABASE.MYSQL_URL.as_str()
    };

    #[cfg(not(debug_assertions))]
    let mysql_url = {
        debug!("rbatis pool init ({})...", CONFIG.DATABASE.MYSQL_URL);
        CONFIG.DATABASE.MYSQL_URL.as_str()
    };

    let rbatis = Rbatis::new();
    rbatis
        .init(MysqlDriver {}, mysql_url)
        .expect("rbatis pool init fail!");

    rbatis
}

pub fn init_mssql_pool() -> Rbatis {
    #[cfg(debug_assertions)]
    let mssql_url = {
        debug!("rbatis pool init ({})...", CONFIG.DATABASE.MSSQL_URL);
        CONFIG.DATABASE.MSSQL_URL.as_str()
    };

    #[cfg(not(debug_assertions))]
    let mssql_url = {
        debug!("rbatis pool init ({})...", CONFIG.DATABASE.MSSQL_URL);
        CONFIG.DATABASE.MSSQL_URL.as_str()
    };

    // #[cfg(not(debug_assertions))]
    // let db_url = {
    //     env!("ACTIX_DB_URL", r#"Environment variable "ACTIX_DB_URL" not found!"#)
    // };

    let rbatis = Rbatis::new();
    rbatis
        .init(MssqlDriver {}, mssql_url)
        .expect("rbatis pool init fail!");

    rbatis
}
