use std::{net::SocketAddr, str::FromStr};

use envman::EnvMan;

#[derive(EnvMan, Debug, PartialEq)]
struct TestNormal {
    /// When retrieving environment variables, they are changed to uppercase.
    db_url: String,
    #[envman(parser = SocketAddr::from_str)]
    db_ip: SocketAddr,
    db_max_conn: u8,
}

#[test]
fn normal() {
    std::env::set_var("DB_URL", "mysql://example");
    std::env::set_var("DB_IP", "127.0.0.1:80");
    std::env::set_var("DB_MAX_CONN", "5");

    assert_eq!(
        TestNormal::load().unwrap(),
        TestNormal {
            db_url: String::from("mysql://example"),
            db_ip: SocketAddr::from_str("127.0.0.1:80").unwrap(),
            db_max_conn: 5
        }
    );
}
