#![doc = include_str!(concat!("../", std::env!("CARGO_PKG_README")))]
pub use envman_derive::EnvMan;

pub trait EnvMan {
    /// Load environment variables
    fn load() -> Result<Self, EnvManError>
    where
        Self: std::marker::Sized;
}

/// Error type for [`EnvMan`]
#[derive(thiserror::Error, Debug)]
pub enum EnvManError {
    /// Failed to read environment variable
    #[error("failed to read environment variable of {key}")]
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
