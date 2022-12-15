use crate::model::TblWarning;
use rbatis::{executor::Executor, rbdc::datetime::FastDateTime, Rbatis};
// use rbs::to_value;

/// 查询设备报警数据
#[html_sql("mapper/masteel/device.html")]
pub async fn select_list(
    rb: &mut dyn Executor,
    start_time: &FastDateTime,
    end_time: &FastDateTime,
    eq_area: &str,
    eq_name: &str,
    status: &str,
    level: u8,
    time_min: u32,
    order_by: &str,
) -> Result<Option<Vec<TblWarning>>, rbatis::Error> {
    // rb.fetch_decode(
    //     r#" SELECT * FROM tbl_warning WHERE fd_wnlevel >= ? "#,
    //     vec![to_value!(level)],
    // ).await
    impled!()
}

/// 查询设备报警报表数据
#[html_sql("mapper/masteel/device.html")]
pub async fn select_report_list(
    rb: &mut dyn Executor,
    start_time: &FastDateTime,
    end_time: &FastDateTime,
    eq_area: &str,
    eq_name: &str,
    status: &str,
    level: u8,
    time_min: u32,
    order_by: &str,
) -> Result<Option<Vec<TblWarning>>, rbatis::Error> {
    impled!()
}
