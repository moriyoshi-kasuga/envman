# envman

## About

This crate adds a macro for easy management of environment variables.

## Install

```toml
[dependencies]
envman = { git = "https://github.com/moriyoshi-kasuga/envman", branch = "main", version = "0.2" }
```

## Example

```rust
use envman::EnvMan;

// The type of field can be set if FromStr is implemented
#[derive(EnvMan)]
struct Foo {
  f0: i32,
  #[envman(rename = "f1")]
  f_1: u8,
  #[envman(default = "default value")]
  f_n: String,
  f_o: Option<i32>
}

let foo = Foo::load().unwrap();
// If rename is not set, it will be an upper case
let f0 = foo.f0; // This value is taken from “F0”.
let f_1 = foo.f_1; // This value is taken from “f1”.
let f_n = foo.f_n; // This value is taken from “F_N” and if it is not set, it will be set to “default value”.
let f_o = foo.f_o; // This value is taken from “F_O” and if it is not set, it will be set to None.
```

## Usecase

```rust
use std::sync::LazyLock;

use envman::EnvMan;

#[tokio::main]
async fn main() {
    // initialize
    let _ = &*ENVIRONMENTS;

    println!("API_URL: {}", ENVIRONMENTS.api_url);
}

pub static ENVIRONMENTS: LazyLock<Environments> = LazyLock::new(|| Environments::load().unwrap());

#[derive(EnvMan)]
pub struct Environments {
    #[envman(default = "https://api.example.com")]
    api_url: String,
}
```
