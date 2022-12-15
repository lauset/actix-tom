use crate::model::WeaSysUser;
use rbatis::Rbatis;
use rbs::to_value;
// use crate::model::user::{NewUser, UpdateUser, User, UserTable, UserFollow, LoginCredentials};

/// 通过 uid 查询正常用户
pub async fn select_day_by_num(
    rb: &Rbatis,
    uid: u32,
) -> Result<Option<WeaSysUser>, rbatis::Error> {
    info!("select_day_by_num uid: {}", uid);
    rb.fetch_decode(
        r#" SELECT * FROM sys_user WHERE user_id = ? "#,
        vec![to_value!(uid)],
    )
    .await
}
