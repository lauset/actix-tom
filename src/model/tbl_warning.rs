use rbatis::rbdc::datetime::FastDateTime;
use rbatis::Rbatis;
use rbatis::{Error, crud, impl_select};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct TblWarningQuery {
    pub eq_area: Option<String>,          // 自动线名称
    pub eq_name: Option<String>,          // 设备名称
    pub level: Option<u8>,                // 等级过滤
    pub start_time: Option<FastDateTime>, // 开始时间
    pub end_time: Option<FastDateTime>,   // 结束时间
    pub time_min: Option<u32>,            // 分钟过滤
    pub status: Option<String>,           // 状态过滤
    pub order_by: Option<String>,         // 排序(等级正倒叙、时间正倒叙)
}

#[derive(Debug)]
pub struct NewTask {
    pub description: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TblWarning {
    pub fd_rowid: u32,
    pub fd_sid: Option<String>,
    pub fd_ruleid: Option<String>,
    pub fd_eqarea: Option<String>,
    pub fd_eqname: Option<String>,
    pub fd_wnname: Option<String>,
    pub fd_wndes: Option<String>,
    pub fd_wnlevel: Option<String>,
    pub fd_starttime: Option<FastDateTime>,
    pub fd_endtime: Option<FastDateTime>,
    pub fd_status: Option<String>,
    pub fd_remarks: Option<String>,
    pub fd_ttid: Option<String>,
    // custom field
    pub fd_sec: Option<u32>,
    pub fd_min: Option<u32>,
    pub fd_hour: Option<u32>,
    pub fd_time: Option<String>,
}
// crud!(TblWarning {}); //crud = insert+select_by_column+update_by_column+delete_by_column

impl_select!(TblWarning{select_device_by_eqarea(name: &str) => "`where fd_eqarea = #{name}`"});

impl TblWarning {
    pub async fn all(rb: &Rbatis) -> Result<Vec<TblWarning>, Error> {
        let mut res: Vec<TblWarning> = Vec::with_capacity(1);
        Ok(res)
    }
}
