#[macro_use]
extern crate lazy_static;

pub mod db;
pub mod http;

pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
