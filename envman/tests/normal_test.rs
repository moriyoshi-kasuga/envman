use envman::EnvMan;

#[derive(EnvMan, Debug, PartialEq)]
struct TestNormal {
    /// When retrieving environment variables, they are changed to uppercase.
    db_url: String,
    db_max_conn: u8,
}

#[test]
fn normal() {
    unsafe {
        std::env::set_var("DB_URL", "mysql://example");
        std::env::set_var("DB_MAX_CONN", "5");
    }
    assert_eq!(
        TestNormal::load().unwrap(),
        TestNormal {
            db_url: String::from("mysql://example"),
            db_max_conn: 5
        }
    );
}
