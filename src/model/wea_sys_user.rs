use rbatis::rbdc::datetime::FastDateTime;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct WeaSysUserTable {
    pub user_id: u32,
    pub user_name: String,
    pub nick_name: String,
    pub password: String,
    pub create_by: Option<String>,
    pub create_time: Option<FastDateTime>,
    pub delete_flag: u8,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct WeaSysUser {
    pub user_id: u32,
    pub user_name: String,
    pub nick_name: String,
    pub password: String,
    pub create_by: Option<String>,
    pub create_time: Option<FastDateTime>,
    pub delete_flag: u8,
}
