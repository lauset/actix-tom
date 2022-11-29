// use rbatis::Rbatis;
// use sqlx::mssql::{MssqlPool, MssqlPoolOptions};

// use crate::model::TblWarning;

// pub async fn init_pool(database_url: &str) -> Result<MssqlPool, sqlx::Error> {
//     MssqlPoolOptions::new()
//         .acquire_timeout(std::time::Duration::from_secs(1))
//         .connect(database_url)
//         .await
// }

// pub async fn get_all_tw(rb: &Rbatis) -> Result<Vec<TblWarning>, &'static str> {
//     TblWarning::all(rb)
//         .await
//         .map_err(|_| "Error get all table warnings")
// }
