#![doc = include_str!("../../README.md")]

pub use envman_derive::EnvMan;
use thiserror::Error;

pub trait EnvMan {
    // fn load() -> &'static [&'static str];
    fn load() -> Result<Self, EnvManError>
    where
        Self: std::marker::Sized;
}

#[derive(Error, Debug)]
pub enum EnvManError {
    #[error("failed to read environment variable of {0}")]
    NotFound(#[from] std::env::VarError),
    #[error("failed to parse environment variable of {key}")]
    Parse { key: String },
}
