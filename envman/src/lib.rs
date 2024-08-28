//! Derive `EnvMan` trait to extract struct field names as a slice.
//!
//!
//! ```rust
//! #   unsafe {
//! #       std::env::set_var("F0", "5");
//! #       std::env::set_var("f1", "3");
//! #       std::env::set_var("F_N", "10");
//! #   }
//!
//! use envman::EnvMan;
//!
//! // The type of field can be set if FromStr is implemented
//! #[derive(EnvMan)]
//! struct Foo {
//!     f0: i32,
//!     #[envman(rename = "f1")]
//!     f_1: u8,
//!     f_n: String,
//!  }
//!
//!  let foo = Foo::load().unwrap();
//!  // If rename is not set, it will be an upper case
//!  let f0 = foo.f0; // This value is taken from “F0”.
//!  let f_1 = foo.f_1; // This value is taken from “f1”.
//!  let f_n = foo.f_n; // This value is taken from “F_N”.
//!
//! ```
pub use envman_derive::EnvMan;
use thiserror::Error;

pub trait EnvMan {
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
