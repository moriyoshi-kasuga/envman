use envman::EnvMan;

#[derive(EnvMan, Debug, PartialEq)]
#[envman(rename_all = "kebab-case", prefix = "db-", suffix = "-main")]
struct TestRenameAll {
    url: String,
}

#[test]
fn rename_all() {
    unsafe {
        std::env::set_var("db-url-main", "mysql://example.1");
    }
    assert_eq!(
        TestRenameAll::load().unwrap(),
        TestRenameAll {
            url: String::from("mysql://example.1"),
        }
    );
}
