use envman::EnvMan;

#[derive(EnvMan, Debug, PartialEq)]
struct TestDefault {
    /// If the variable could not be taken, this is the value
    #[envman(default = "redis://example".to_string())]
    redis_url: String,
    #[envman(default = "5", alltime_parse)]
    redis_max_conn: u8,
}

#[test]
fn default() {
    assert_eq!(
        TestDefault::load().unwrap(),
        TestDefault {
            redis_url: String::from("redis://example"),
            redis_max_conn: 5
        }
    );
}
