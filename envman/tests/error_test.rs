use envman::EnvMan;

#[derive(EnvMan, Debug)]
#[envman(prefix = "ERROR_")]
pub struct ErrorConfig {
    pub port: u16,
    pub count: i32,
}

#[test]
fn parse_error_contains_details() {
    std::env::set_var("ERROR_PORT", "not_a_number");
    std::env::set_var("ERROR_COUNT", "100");

    let result = ErrorConfig::load_from_env();

    assert!(result.is_err());

    let err = result.unwrap_err();
    let err_string = format!("{}", err);

    // Error should match exact format
    assert_eq!(err_string, "failed to parse environment variable 'ERROR_PORT' with value 'not_a_number' (expected type: error_test::ErrorConfig)");

    std::env::remove_var("ERROR_PORT");
    std::env::remove_var("ERROR_COUNT");
}

#[test]
fn not_found_error_contains_key() {
    #[derive(EnvMan, Debug)]
    #[envman(prefix = "MISSING_")]
    pub struct MissingConfig {
        pub required_field: String,
    }

    let result = MissingConfig::load_from_env();

    assert!(result.is_err());

    let err = result.unwrap_err();
    let err_string = format!("{}", err);

    // Error should match exact format
    assert_eq!(
        err_string,
        "failed to read environment variable 'MISSING_REQUIRED_FIELD'"
    );
}
