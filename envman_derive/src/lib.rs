use syn::{parse_macro_input, DeriveInput};

/**
Automatically implements [`envman::EnvMan`]


# Note
- if the field is Option, the default value is None
- if the field has rename attribute, the field name is not affected by suffix, prefix and rename_all

# Struct Attributes:

### rename_all: `rename_all = "rule"` (default: SCREAMING-KEBAB-CASE)
applies to all fields

The possible values are ("lowercase", "UPPERCASE", "PascalCase", "camelCase",
"snake_case", "SCREAMING_SNAKE_CASE", "kebab-case", "SCREAMING-KEBAB-CASE")

### prefix: `prefix = "prefix"` (default: None)
prefix to all fields

### suffix: `suffix = "suffix"` (default: None)
suffix to all fields

# Field Attributes:

### rename : `rename = "new name"` (default: UPPER_CASE)

### parser: `parser = utils::default_parser` (default: FromStr::from_str)
parser type is `fn(&str) -> Result<T, E>` and `E` has impl `std::error::Error`

### group_test: (default: None)
if under test, use this value (Priority is first)

- test_flag: `test` (Equivalent to the code below)
- test_expr: `test = Default::default()` (put anything of expr)

###  group_default: (default: None)
if not found in env, use this value
if a test exists and is under test, use the test

- default_flag: `default` (Equivalent to the code below)
- default_expr: `default = Default::default()` (put anything of expr)

### alltime_parse: `alltime_parse` (default: false)
The normal default (and test) return value is the field type
if this is set, the return value is a string and the parser is used

### nest: `nest` (default: false)
If the field is impl envman::EnvMan, it will be parsed as a struct

# Example
```rust
use envman::EnvMan;

#[derive(EnvMan)]
struct Foo {
  f0: i32,
  #[envman(rename = "f1")]
  f_1: u8,
  #[envman(default = "default value".to_string())]
  f_n: String,
  f_o: Option<i32>,
  #[envman(default = 1, test = 2)]
  f_test: u8,
  #[envman(nest, default)]
  db: Option<DbData>,
}

#[derive(EnvMan, Default)]
#[envman(prefix = "DB_")]
struct DbData {
  url: String,
}

unsafe {
    std::env::set_var("F0", "1");
    std::env::set_var("f1", "2");
    std::env::set_var("DB_URL", "url");
}

let foo = Foo::load().unwrap();
assert_eq!(foo.f0, 1);
assert_eq!(foo.f_1, 2);
assert_eq!(foo.f_n, "default value");
assert_eq!(foo.f_o, None);
// in doctest, cfg is doctest, not test
assert_eq!(foo.f_test, 1);
assert_eq!(foo.db.unwrap().url, "url");
*/
#[proc_macro_derive(EnvMan, attributes(envman))]
pub fn derive_envman(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    envman_derive_internals::derive_envman(parse_macro_input!(input as DeriveInput))
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
