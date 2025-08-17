# [EnvMan][docsrs]: Environments (variables) Manager

[![EnvMan on crates.io][cratesio-image]][cratesio]
[![EnvMan on docs.rs][docsrs-image]][docsrs]

[cratesio-image]: https://img.shields.io/crates/v/envman.svg
[cratesio]: https://crates.io/crates/envman
[docsrs-image]: https://docs.rs/envman/badge.svg
[docsrs]: https://docs.rs/envman

EnvMan is a Rust crate that provides a procedural macro to simplify the management of environment variables. It allows you to automatically load and parse environment variables into your Rust structs, with support for default values, custom parsers, and more.

## Features

- **Automatic Environment Variable Loading**: Automatically load environment variables into struct fields.
- **Customizable Field Attributes**: Use attributes to customize field names, parsers, default values, and nested structures efficiently, now with centralized error handling.
- **Improved Parsing Mechanism**: Attributes are parsed using a unified configuration builder for better error reporting and validation.
- **Enhanced Error Context**: Provides precise error messages, including problematic field names and attributes.
- **Support for Nested Structs**: Easily manage nested configurations with support for nested structs.
- **Flexible Naming Conventions**: Use `rename_all`, `prefix`, and `suffix` to control environment variable naming.

## Usage

Here's a basic example demonstrating how to use EnvMan to manage environment variables:

```rust
use envman::EnvMan;
use std::net::IpAddr;

#[derive(EnvMan)]
struct Config {
    #[envman(rename = "APP_PORT", test = 8080)]
    port: u16,
    #[envman(nest)]
    database: DatabaseConfig,
}

#[derive(EnvMan)]
#[envman(prefix = "DB_",)]
struct DatabaseConfig {
    #[envman(default = "127.0.0.1", alltime_parse)]
    host: IpAddr,
    #[envman(default = 5432)]
    port: u16,
}

// NOTE: This is for demonstration purposes only in README.
// In real applications, set environment variables through your system or .env files.
#[allow(unused_unsafe)]
unsafe {
    std::env::set_var("APP_PORT", "5000");
    std::env::set_var("DB_HOST", "192.168.1.1");
    std::env::set_var("DB_PORT", "5432");
}

// Load the configuration from environment variables
let config = Config::load().expect("Failed to load configuration");

// Assertions to verify the configuration
assert_eq!(config.port, 5000);
assert_eq!(config.database.host.to_string(), "192.168.1.1");
assert_eq!(config.database.port, 5432);
```

## Attributes

### Struct Attributes

- **`rename_all`**: Apply a naming convention to all fields (default: `SCREAMING_SNAKE_CASE`).
- **`prefix`**: Add a prefix to all field names.
- **`suffix`**: Add a suffix to all field names.

### Field Attributes

- **`rename`**: Specify a custom environment variable name for a field.
- **`default`**: Provide a default value if the environment variable is not set.
- **`parser`**: Use a custom parser function to parse the environment variable. (default: `FromStr::from_str`)
- **`nest`**: Indicate that the field is a nested struct implementing `EnvMan`.

## More Info

more info: [doc.rs](https://docs.rs/envman/latest/envman/derive.EnvMan.html)

## License

Licensed under

- [MIT license](https://github.com/moriyoshi-kasuga/envman/blob/main/LICENSE)
