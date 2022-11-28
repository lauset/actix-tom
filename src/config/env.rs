#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct EnvConfig {
    // actix 绑定的 host
    pub BIND_HOST: String,
    // actix 绑定的端口
    pub PORT: u32,
    // 数据库 host
    pub MSSQL_URL: String,
    pub MYSQL_URL: String,
}

///默认配置
impl Default for EnvConfig {
    fn default() -> Self {
        let yml_data = include_str!("../../env.yaml");
        let result: EnvConfig =
            serde_yaml::from_str(yml_data).expect("load env.yaml config file fail");
        // if result.debug {
        //     println!("[abs_admin] load config:{:?}", result);
        //     println!("[abs_admin] ///////////////////// Start On Debug Mode ////////////////////////////");
        // } else {
        //     println!("[abs_admin] ///////////////////// Start On Release Mode ////////////////////////////");
        // }
        result
    }
}
