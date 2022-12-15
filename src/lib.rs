#![feature(option_result_contains)]
#![feature(is_some_and)]
#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate rbatis;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;

pub mod controller {
    pub mod example;
    pub mod masteel;
    pub mod weather;
}

pub mod error {
    pub mod index;
    pub mod service;
}

pub mod model;

pub mod service {
    pub mod masteel;
    pub mod weather;
}

pub mod config;
pub mod constants;
pub mod db;

pub use config::log as app_log;
pub use config::rb;
pub use config::CONFIG;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
