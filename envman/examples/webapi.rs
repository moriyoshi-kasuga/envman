#![allow(clippy::print_stdout, clippy::unwrap_used)]

use std::net::SocketAddr;

use envman::EnvMan;

fn main() {
    std::env::set_var("JWT_SECRET", "secret");

    // initialize
    let environments = Environments::load().unwrap();

    println!("API_URL: {}", environments.api_url);
}

#[derive(EnvMan)]
pub struct Environments {
    #[envman(default = "127.0.0.1:8080")]
    pub api_url: SocketAddr,
    #[envman(test = "secret")]
    pub jwt_secret: String,
}
