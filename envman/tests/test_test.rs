use envman::EnvMan;

#[derive(EnvMan, Debug, PartialEq)]
struct TestTestValue {
    /// Apply value only at test time
    #[envman(test = "2".to_string())]
    secret_1: String,
    /// Test values take precedence over default values
    #[envman(default = "1".to_string(), test = "3".to_string())]
    secret_2: String,
}

#[test]
fn test_value() {
    assert_eq!(
        TestTestValue::load().unwrap(),
        TestTestValue {
            secret_1: String::from("2"),
            secret_2: String::from("3")
        }
    );
}
