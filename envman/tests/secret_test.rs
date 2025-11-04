use envman::{EnvMan, EnvManDebug};

#[derive(EnvMan, EnvManDebug)]
#[envman(prefix = "SECRET_")]
pub struct SecretConfig {
    username: String,

    #[envman(secret)]
    pub password: String,

    #[envman(secret)]
    pub api_key: String,

    host: String,
}

#[test]
fn secret_masking_debug() {
    std::env::set_var("SECRET_USERNAME", "admin");
    std::env::set_var("SECRET_PASSWORD", "super_secret_pass");
    std::env::set_var("SECRET_API_KEY", "sk-1234567890abcdef");
    std::env::set_var("SECRET_HOST", "localhost");

    let config = SecretConfig::load_from_env().unwrap();

    let debug_output = format!("{:?}", config);

    // Verify exact debug output format
    assert_eq!(
        debug_output,
        "SecretConfig { username: \"admin\", password: \"***\", api_key: \"***\", host: \"localhost\" }"
    );

    std::env::remove_var("SECRET_USERNAME");
    std::env::remove_var("SECRET_PASSWORD");
    std::env::remove_var("SECRET_API_KEY");
    std::env::remove_var("SECRET_HOST");
}

#[derive(EnvMan, EnvManDebug)]
#[envman(prefix = "OPTIONAL_SECRET_")]
pub struct OptionalSecretConfig {
    #[envman(secret)]
    pub token: Option<String>,
}

#[test]
fn optional_secret_masking() {
    // Test with value
    std::env::set_var("OPTIONAL_SECRET_TOKEN", "secret_token_123");
    let config = OptionalSecretConfig::load_from_env().unwrap();
    let debug_output = format!("{:?}", config);

    // Verify exact format with Some value masked
    assert_eq!(debug_output, "OptionalSecretConfig { token: \"***\" }");

    std::env::remove_var("OPTIONAL_SECRET_TOKEN");

    // Test without value
    let config = OptionalSecretConfig::load_from_env().unwrap();
    let debug_output = format!("{:?}", config);

    // Verify exact format with None value masked
    assert_eq!(debug_output, "OptionalSecretConfig { token: \"***\" }");
}
