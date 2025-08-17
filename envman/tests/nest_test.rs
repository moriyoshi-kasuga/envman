use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    str::FromStr,
};

use envman::EnvMan;

#[derive(EnvMan, Debug, PartialEq)]
struct TestNest {
    server_ip: SocketAddr,
    #[envman(nest, default)]
    backend: Option<BackEndConfig>,
}

#[derive(EnvMan, Debug, PartialEq)]
struct BackEndConfig {
    backend_ip: SocketAddr,
    backend_kind: u8,
}

impl Default for BackEndConfig {
    fn default() -> Self {
        Self {
            backend_ip: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3000),
            backend_kind: Default::default(),
        }
    }
}

#[test]
fn nest() {
    std::env::set_var("SERVER_IP", "127.0.0.1:80");
    std::env::set_var("BACKEND_IP", "127.0.0.1:5000");

    // Not Set 'BACKEND_KIND', backend is Default
    assert_eq!(
        TestNest {
            server_ip: SocketAddr::from_str("127.0.0.1:80").unwrap(),
            backend: Some(BackEndConfig {
                backend_ip: SocketAddr::from_str("127.0.0.1:3000").unwrap(),
                backend_kind: 0
            })
        },
        TestNest::load().unwrap()
    );

    std::env::set_var("BACKEND_KIND", "5");

    assert_eq!(
        TestNest {
            server_ip: SocketAddr::from_str("127.0.0.1:80").unwrap(),
            backend: Some(BackEndConfig {
                backend_ip: SocketAddr::from_str("127.0.0.1:5000").unwrap(),
                backend_kind: 5
            })
        },
        TestNest::load().unwrap()
    );
}
