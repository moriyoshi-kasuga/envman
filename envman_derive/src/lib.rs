#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stdout,
    clippy::print_stderr
)]

use syn::{parse_macro_input, DeriveInput};

/// Automatically implements [`envman::EnvMan`]
///
/// # Note
/// - If the field is `Option`, the default value is `None`.
/// - If the field has a `rename` attribute, the field name is not affected by `suffix`, `prefix`, and `rename_all`.
/// - The `rename_all` attribute affects only the base field name transformation and does not influence the application of `prefix` or `suffix`.
///
/// # Struct Attributes:
///
/// ### rename_all: `rename_all = "rule"` (default: SCREAMING_SNAKE_CASE)
/// Applies to all fields.
///
/// The possible values are ("lowercase", "UPPERCASE", "PascalCase", "camelCase",
/// "snake_case", "SCREAMING_SNAKE_CASE", "kebab-case", "SCREAMING-KEBAB-CASE").
///
/// ### prefix: `prefix = "prefix"` (default: None)
/// Prefix to all fields.
///
/// ### suffix: `suffix = "suffix"` (default: None)
/// Suffix to all fields.
///
/// # Field Attributes:
///
/// ### rename : `rename = "new name"` (default: UPPER_CASE)
///
/// ### parser: `parser = utils::default_parser` (default: FromStr::from_str)
/// Parser type is `fn(&str) -> Result<T, E>` and `E` must implement `std::error::Error`.
///
/// ### separator: `separator = ","` (default: None)
/// For Vec/array types, split the environment variable by this separator.
/// Example: `ALLOWED_HOSTS=host1,host2,host3` becomes `vec!["host1", "host2", "host3"]`
///
/// ### validate: `validate = my_validator` (default: None)
/// Custom validation function with signature `fn(&T) -> Result<(), E>` where `E: Display`.
/// The error message from the Result will be included in the validation error.
///
/// ### secret: `secret` (default: false)
/// Mark this field as secret. When used with `EnvManDebug`, the value will be masked as "***".
///
/// ### group_test: (default: None)
/// If under test, use this value (Priority is first).
///
/// - test_flag: `test` (Equivalent to the code below)
/// - test_expr: `test = Default::default()` (put any expression)
///
/// ### group_default: (default: None)
/// If not found in the environment, use this value.
/// If a test exists and is under test, use the test.
///
/// - default_flag: `default` (Equivalent to the code below)
/// - default_expr: `default = Default::default()` (put any expression)
///
/// ### nest: `nest` (default: false)
/// If the field implements `envman::EnvMan`, it will be parsed as a struct.
///
/// # Example
/// ```rust
/// # use envman_derive::EnvMan;
/// # mod envman {
/// #   include!("../../envman/src/def.rs");
/// # }
/// # use envman::EnvMan;
///
/// #[derive(EnvMan)]
/// struct Foo {
///   normal: i32,
///   #[envman(rename = "renamed")]
///   so_long_name: u8,
///   #[envman(default = "default value")]
///   default: String,
///   wrapped: Option<i32>,
///   #[envman(default = 1, test = 2)]
///   test_value: u8,
///   #[envman(nest)]
///   nested: DbData,
///   #[envman(separator = ",")]
///   allowed_hosts: Vec<String>,
/// }
///
/// #[derive(EnvMan, Default)]
/// #[envman(prefix = "DB_")]
/// struct DbData {
///   url: String,
/// }
///
/// #[allow(unused_unsafe)]
/// unsafe {
///     std::env::set_var("NORMAL", "1");
///     // rename attribute is not affected by rename_all, prefix, and suffix
///     std::env::set_var("renamed", "2");
///     std::env::set_var("DB_URL", "url");
///     std::env::set_var("ALLOWED_HOSTS", "host1,host2,host3");
/// }
///
/// let foo = Foo::load_from_env().unwrap();
/// assert_eq!(foo.normal, 1);
/// assert_eq!(foo.so_long_name, 2);
/// assert_eq!(foo.default, "default value");
/// assert_eq!(foo.wrapped, None);
/// // in doctest, cfg is doctest, not test
/// assert_eq!(foo.test_value, 1);
/// assert_eq!(foo.nested.url, "url");
/// assert_eq!(foo.allowed_hosts, vec!["host1", "host2", "host3"]);
/// ```
#[proc_macro_derive(EnvMan, attributes(envman))]
pub fn derive_envman(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    envman_derive_internals::derive_envman(parse_macro_input!(input as DeriveInput))
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Derives Debug with masking support for fields marked with `#[envman(secret)]`
///
/// # Example
/// ```rust
/// # use envman_derive::{EnvMan, EnvManDebug};
/// # mod envman {
/// #   include!("../../envman/src/def.rs");
/// # }
/// # use envman::EnvMan;
///
/// #[derive(EnvMan, EnvManDebug)]
/// struct Config {
///     username: String,
///     #[envman(secret)]
///     password: String,
///     #[envman(secret)]
///     api_key: String,
/// }
///
/// #[allow(unused_unsafe)]
/// unsafe {
///     std::env::set_var("USERNAME", "admin");
///     std::env::set_var("PASSWORD", "secret123");
///     std::env::set_var("API_KEY", "key_123456");
/// }
///
/// let config = Config::load_from_env().unwrap();
/// let debug_output = format!("{:?}", config);
/// assert!(debug_output.contains("username: \"admin\""));
/// assert!(debug_output.contains("password: \"***\""));
/// assert!(debug_output.contains("api_key: \"***\""));
/// ```
#[proc_macro_derive(EnvManDebug, attributes(envman))]
pub fn derive_envman_debug(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    envman_derive_internals::derive_envman_debug(parse_macro_input!(input as DeriveInput))
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
