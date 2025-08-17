#![doc = include_str!(concat!("../", std::env!("CARGO_PKG_README")))]
#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stdout,
    clippy::print_stderr
)]

#[cfg(feature = "derive")]
pub use envman_derive::EnvMan;

mod def;
pub use def::*;
