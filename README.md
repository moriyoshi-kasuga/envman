# [EnvMan][docsrs]: Environments (variables) Manager

[![EnvMan on crates.io][cratesio-image]][cratesio]
[![EnvMan on docs.rs][docsrs-image]][docsrs]

[cratesio-image]: https://img.shields.io/crates/v/envman.svg
[cratesio]: https://crates.io/crates/envman
[docsrs-image]: https://docs.rs/envman/badge.svg
[docsrs]: https://docs.rs/envman

This crate adds a macro for easy management of environment variables.

If you would like to see more details, please see [these codes](https://github.com/moriyoshi-kasuga/envman/tree/main/envman/tests).

## Example

```rust
use envman::EnvMan;

unsafe {
  std::env::set_var("F0", "-1");
  std::env::set_var("f1", "1");
}

// The type of field can be set if FromStr is implemented
#[derive(EnvMan)]
struct Foo {
  f0: i32,
  #[envman(rename = "f1")]
  f_1: u8,
  #[envman(default = "default value")]
  f_n: String,
  f_o: Option<i32>,
  #[envman(default = "1", test = "2")]
  f_test: u8,
}

// If rename is not set, it will be an upper case
let foo = Foo::load().unwrap();
// This value is taken from “F0”.
let f0 = foo.f0;
// This value is taken from “f1”.
let f_1 = foo.f_1;
// This value is taken from “F_N” and if it is not set, it will be set to “default value”.
let f_n = foo.f_n;
// This value is taken from “F_O” and if it is not set, it will be set to None.
let f_o = foo.f_o;
// This value is taken from “F_TEST” and if it is not set, it will be set to 1.
// and if it under test, it will be set to 2.
let f_test = foo.f_test;
```

## Usecase

```rust
use std::sync::LazyLock;

use envman::EnvMan;

fn main() {
    // this unsafe block is necessary for test
    // and it is not necessary in production
    unsafe {
      std::env::set_var("JWT_SECRET", "secret");
    }

    // initialize
    let _ = &*ENVIRONMENTS;

    println!("API_URL: {}", ENVIRONMENTS.api_url);
}

pub static ENVIRONMENTS: LazyLock<Environments> = LazyLock::new(|| Environments::load().unwrap());

#[derive(EnvMan)]
pub struct Environments {
    #[envman(default = "https://api.example.com")]
    api_url: String,
    #[envman(test = "secret")]
    jwt_secret: String,
}
```

## License

Licensed under

- [MIT license](https://github.com/moriyoshi-kasuga/envman/blob/main/LICENSE)
