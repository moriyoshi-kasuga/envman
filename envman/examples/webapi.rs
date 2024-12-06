use std::{net::SocketAddr, sync::LazyLock};

use envman::EnvMan;

fn main() {
    // this unsafe block is necessary for test
    // and it is not necessary in production
    unsafe {
        std::env::set_var("JWT_SECRET", "secret");
    }

    // initialize
    let _ = &*ENVIRONMENTS;

    println!("API_URL: {}", ENVIRONMENTS.api_url);
}

pub static ENVIRONMENTS: LazyLock<Environments> = LazyLock::new(|| Environments::load().unwrap());

#[derive(EnvMan)]
pub struct Environments {
    #[envman(default = "127.0.0.1:8080", alltime_parse)]
    pub api_url: SocketAddr,
    #[envman(test = "secret".to_string())]
    pub jwt_secret: String,
}
