use envman::EnvMan;

// Example: Email validation with custom error
#[allow(clippy::ptr_arg)]
fn validate_email(email: &String) -> Result<(), String> {
    if !email.contains('@') {
        Err(format!("'{}' is not a valid email address", email))
    } else {
        Ok(())
    }
}

// Example: Range validation with detailed error
fn validate_thread_count(count: &u32) -> Result<(), String> {
    if *count < 1 {
        Err("Thread count must be at least 1".to_string())
    } else if *count > 1024 {
        Err(format!("Thread count {} exceeds maximum of 1024", count))
    } else {
        Ok(())
    }
}

// Example: URL validation
#[allow(clippy::ptr_arg)]
fn validate_url(url: &String) -> Result<(), String> {
    if !url.starts_with("http://") && !url.starts_with("https://") {
        Err(format!("URL '{}' must start with http:// or https://", url))
    } else {
        Ok(())
    }
}

// Example: Non-empty string validation
#[allow(clippy::ptr_arg)]
fn validate_non_empty(s: &String) -> Result<(), String> {
    if s.trim().is_empty() {
        Err("Value cannot be empty".to_string())
    } else {
        Ok(())
    }
}

#[derive(EnvMan)]
#[envman(prefix = "APP_")]
struct AdvancedConfig {
    #[envman(validate = validate_email)]
    admin_email: String,

    #[envman(validate = validate_thread_count, default = 4)]
    thread_count: u32,

    #[envman(validate = validate_url)]
    api_url: String,

    #[envman(validate = validate_non_empty)]
    app_name: String,
}

#[test]
fn test_valid_config() {
    std::env::set_var("APP_ADMIN_EMAIL", "admin@example.com");
    std::env::set_var("APP_THREAD_COUNT", "8");
    std::env::set_var("APP_API_URL", "https://api.example.com");
    std::env::set_var("APP_APP_NAME", "MyApp");

    let config = AdvancedConfig::load_from_env().unwrap();

    assert_eq!(config.admin_email, "admin@example.com");
    assert_eq!(config.thread_count, 8);
    assert_eq!(config.api_url, "https://api.example.com");
    assert_eq!(config.app_name, "MyApp");

    std::env::remove_var("APP_ADMIN_EMAIL");
    std::env::remove_var("APP_THREAD_COUNT");
    std::env::remove_var("APP_API_URL");
    std::env::remove_var("APP_APP_NAME");
}

#[test]
fn test_invalid_email() {
    std::env::set_var("APP_ADMIN_EMAIL", "not-an-email");
    std::env::set_var("APP_THREAD_COUNT", "8");
    std::env::set_var("APP_API_URL", "https://api.example.com");
    std::env::set_var("APP_APP_NAME", "MyApp");

    let result = AdvancedConfig::load_from_env();
    assert!(result.is_err());

    if let Err(err) = result {
        let err_msg = format!("{}", err);
        assert_eq!(
            err_msg,
            "validation failed for environment variable 'APP_ADMIN_EMAIL' with value 'not-an-email': 'not-an-email' is not a valid email address"
        );
    }

    std::env::remove_var("APP_ADMIN_EMAIL");
    std::env::remove_var("APP_THREAD_COUNT");
    std::env::remove_var("APP_API_URL");
    std::env::remove_var("APP_APP_NAME");
}

#[test]
fn test_invalid_thread_count_too_high() {
    std::env::set_var("APP_ADMIN_EMAIL", "admin@example.com");
    std::env::set_var("APP_THREAD_COUNT", "2000");
    std::env::set_var("APP_API_URL", "https://api.example.com");
    std::env::set_var("APP_APP_NAME", "MyApp");

    let result = AdvancedConfig::load_from_env();
    assert!(result.is_err());

    if let Err(err) = result {
        let err_msg = format!("{}", err);
        assert_eq!(
            err_msg,
            "validation failed for environment variable 'APP_THREAD_COUNT' with value '2000': Thread count 2000 exceeds maximum of 1024"
        );
    }

    std::env::remove_var("APP_ADMIN_EMAIL");
    std::env::remove_var("APP_THREAD_COUNT");
    std::env::remove_var("APP_API_URL");
    std::env::remove_var("APP_APP_NAME");
}

#[test]
fn test_invalid_url() {
    std::env::set_var("APP_ADMIN_EMAIL", "admin@example.com");
    std::env::set_var("APP_THREAD_COUNT", "8");
    std::env::set_var("APP_API_URL", "ftp://api.example.com");
    std::env::set_var("APP_APP_NAME", "MyApp");

    let result = AdvancedConfig::load_from_env();
    assert!(result.is_err());

    if let Err(err) = result {
        let err_msg = format!("{}", err);
        assert_eq!(
            err_msg,
            "validation failed for environment variable 'APP_API_URL' with value 'ftp://api.example.com': URL 'ftp://api.example.com' must start with http:// or https://"
        );
    }

    std::env::remove_var("APP_ADMIN_EMAIL");
    std::env::remove_var("APP_THREAD_COUNT");
    std::env::remove_var("APP_API_URL");
    std::env::remove_var("APP_APP_NAME");
}

#[test]
fn test_empty_app_name() {
    std::env::set_var("APP_ADMIN_EMAIL", "admin@example.com");
    std::env::set_var("APP_THREAD_COUNT", "8");
    std::env::set_var("APP_API_URL", "https://api.example.com");
    std::env::set_var("APP_APP_NAME", "   ");

    let result = AdvancedConfig::load_from_env();
    assert!(result.is_err());

    if let Err(err) = result {
        let err_msg = format!("{}", err);
        assert_eq!(
            err_msg,
            "validation failed for environment variable 'APP_APP_NAME' with value '   ': Value cannot be empty"
        );
    }

    std::env::remove_var("APP_ADMIN_EMAIL");
    std::env::remove_var("APP_THREAD_COUNT");
    std::env::remove_var("APP_API_URL");
    std::env::remove_var("APP_APP_NAME");
}

#[test]
fn test_default_thread_count() {
    std::env::set_var("APP_ADMIN_EMAIL", "admin@example.com");
    // Don't set APP_THREAD_COUNT - use default
    std::env::set_var("APP_API_URL", "https://api.example.com");
    std::env::set_var("APP_APP_NAME", "MyApp");

    let config = AdvancedConfig::load_from_env().unwrap();

    assert_eq!(config.thread_count, 4); // Default value

    std::env::remove_var("APP_ADMIN_EMAIL");
    std::env::remove_var("APP_API_URL");
    std::env::remove_var("APP_APP_NAME");
}
