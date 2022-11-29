#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct EnvConfig {
    pub APP: AppConfig,
    pub SERVER: ServerConfig,
    pub DATABASE: DatabaseConfig
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AppConfig {
    pub NAME: String,
    pub VERSION: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ServerConfig {
    pub HOST: String,
    pub PORT: u32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DatabaseConfig {
    pub MSSQL_URL: String,
    pub MYSQL_URL: String,
}

/// 配置项
impl Default for EnvConfig {
    fn default() -> Self {
        let yml_data = include_str!("../../env.yaml");
        let result: EnvConfig =
            serde_yaml::from_str(yml_data).expect("load env.yaml config file fail");
        result
    }
}
