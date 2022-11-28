use self::env::EnvConfig;
use once_cell::sync::Lazy;

pub mod env;
pub mod log;
pub mod rb;

pub static CONFIG: Lazy<EnvConfig> = Lazy::new(EnvConfig::default);
