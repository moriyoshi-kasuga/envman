use envman::EnvMan;

#[derive(EnvMan)]
#[envman(prefix = "ARRAY_")]
struct ArrayConfig {
    #[envman(separator = ",")]
    tags: Vec<String>,

    #[envman(separator = ":")]
    ports: Vec<u16>,

    #[envman(separator = ",", default = vec![1, 2, 3])]
    numbers: Vec<i32>,
}

#[test]
fn array_with_separator() {
    std::env::set_var("ARRAY_TAGS", "rust,cargo,testing");
    std::env::set_var("ARRAY_PORTS", "8080:9090:3000");

    let config = ArrayConfig::load_from_env().unwrap();

    assert_eq!(config.tags, vec!["rust", "cargo", "testing"]);
    assert_eq!(config.ports, vec![8080, 9090, 3000]);
    assert_eq!(config.numbers, vec![1, 2, 3]);

    std::env::remove_var("ARRAY_TAGS");
    std::env::remove_var("ARRAY_PORTS");
}

#[test]
fn array_with_whitespace() {
    std::env::set_var("ARRAY_TAGS", "rust, cargo , testing");
    std::env::set_var("ARRAY_PORTS", "8080:9090:3000");

    let config = ArrayConfig::load_from_env().unwrap();

    // Should trim whitespace
    assert_eq!(config.tags, vec!["rust", "cargo", "testing"]);

    std::env::remove_var("ARRAY_TAGS");
    std::env::remove_var("ARRAY_PORTS");
}

#[test]
fn array_parse_error() {
    std::env::set_var("ARRAY_TAGS", "rust,cargo,testing");
    std::env::set_var("ARRAY_PORTS", "8080:invalid:3000");

    let result = ArrayConfig::load_from_env();
    assert!(result.is_err());

    std::env::remove_var("ARRAY_TAGS");
    std::env::remove_var("ARRAY_PORTS");
}

#[derive(EnvMan)]
#[envman(prefix = "OPTIONAL_ARRAY_")]
struct OptionalArrayConfig {
    #[envman(separator = ",")]
    tags: Option<Vec<String>>,
}

#[test]
fn optional_array() {
    // Test with value
    std::env::set_var("OPTIONAL_ARRAY_TAGS", "a,b,c");
    let config = OptionalArrayConfig::load_from_env().unwrap();
    assert_eq!(
        config.tags,
        Some(vec!["a".to_string(), "b".to_string(), "c".to_string()])
    );
    std::env::remove_var("OPTIONAL_ARRAY_TAGS");

    // Test without value
    let config = OptionalArrayConfig::load_from_env().unwrap();
    assert_eq!(config.tags, None);
}
