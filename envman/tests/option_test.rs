use envman::EnvMan;

#[derive(EnvMan, Debug, PartialEq)]
struct TestOption {
    /// No error is returned even if the value is missing. Returned is None
    secret_1: Option<String>,
    /// Default is available just in case
    #[envman(default = "5".to_string())]
    secret_2: Option<String>,
}

#[test]
fn option() {
    assert_eq!(
        TestOption::load().unwrap(),
        TestOption {
            secret_1: None,
            secret_2: Some(String::from("5"))
        }
    );
}
