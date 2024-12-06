#![doc = include_str!(concat!("../", std::env!("CARGO_PKG_README")))]
pub use envman_derive::EnvMan;

pub trait EnvMan {
    /// Load environment variables
    fn load() -> Result<Self, EnvManError>
    where
        Self: std::marker::Sized;
}

#[derive(thiserror::Error, Debug)]
/// Error type for [`EnvMan`]
pub enum EnvManError {
    #[error("failed to read environment variable of {key}")]
    /// Failed to read environment variable
    NotFound { key: &'static str },
    /// Failed to parse environment variable
    /// I didn't include a value just in case.
    #[error("failed to parse environment variable of {key}")]
    Parse {
        key: &'static str,
        #[source]
        source: Box<dyn std::error::Error>,
    },
}
