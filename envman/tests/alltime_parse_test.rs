use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    str::FromStr,
};

use envman::EnvMan;

#[derive(EnvMan, Debug, PartialEq)]
struct TestNormal {
    /// When retrieving environment variables, they are changed to uppercase.
    #[envman(default = "mysql://localhost")]
    db_url: String,
    #[envman(default = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080))]
    db_ip: SocketAddr,
    #[envman(default = 10)]
    db_max_conn: u8,
}

#[test]
fn normal() {
    std::env::set_var("DB_URL", "mysql://example");
    std::env::set_var("DB_IP", "127.0.0.1:80");

    assert_eq!(
        TestNormal::load().unwrap(),
        TestNormal {
            db_url: String::from("mysql://example"),
            db_ip: SocketAddr::from_str("127.0.0.1:80").unwrap(),
            db_max_conn: 10
        }
    );
}
