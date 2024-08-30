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
  f_n: String,
}

let foo = Foo::load().unwrap();
// If rename is not set, it will be an upper case
let f0 = foo.f0; // This value is taken from “F0”.
let f_1 = foo.f_1; // This value is taken from “f1”.
let f_n = foo.f_n; // This value is taken from “F_N”.
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

static ENVIRONMENTS: LazyLock<Foo> = LazyLock::new(|| Foo::load().unwrap());

#[derive(EnvMan)]
struct Foo {
    #[envman(default = "https://api.example.com")]
    api_url: String,
}
```
