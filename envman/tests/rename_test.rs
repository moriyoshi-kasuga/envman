use envman::EnvMan;

#[derive(EnvMan, Debug, PartialEq)]
struct TestRename {
    /// Rename key of environment
    #[envman(rename = "CORE_DB_URL")]
    db_url_1: String,
    /// The value you change will not change anything.
    /// In other words Lowercase letters are allowed.
    #[envman(rename = "transaction_db_url")]
    db_url_2: String,
}

#[test]
fn rename() {
    std::env::set_var("CORE_DB_URL", "mysql://example.1");
    std::env::set_var("transaction_db_url", "mysql://example.2");

    assert_eq!(
        TestRename::load_from_env().unwrap(),
        TestRename {
            db_url_1: String::from("mysql://example.1"),
            db_url_2: String::from("mysql://example.2")
        }
    );
}
