# Windmark Comments

[![crates.io](https://img.shields.io/crates/v/windmark-comments.svg)](https://crates.io/crates/windmark-comments)
[![docs.rs](https://docs.rs/windmark-comments/badge.svg)](https://docs.rs/windmark-comments)
[![github.com](https://github.com/gemrest/windmark-comments/actions/workflows/check.yaml/badge.svg?branch=main)](https://github.com/gemrest/windmark-comments/actions/workflows/check.yaml)

A comment engine module for Windmark.

## Usage

### Add Windmark Comments as a dependency

```toml
# Cargo.toml

[dependencies]
windmark-comments = "0.1.0"
```

### Attach Windmark Comments as a module

```rust
// src/main.rs

use windmark::Response;

#[windmark::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
  windmark::Router::new()
    .set_private_key_file("windmark_comments_private.pem")
    .set_certificate_file("windmark_comments_public.pem")
    .mount("/", Box::new(|_| Response::Success("Hello, World!".into())))
    .set_error_handler(Box::new(|_| {
      Response::PermanentFailure("This route does not exist!".into())
    }))
    // Attach Windmark Comments
    .attach(windmark_comments::module)
    .run()
    .await
}
```

## Examples

Examples can be found within the
[`examples`](https://github.com/gemrest/windmark-comments/tree/main/examples)
directory.

## License

This project is licensed with the
[GNU General Public License v3.0](https://github.com/gemrest/windmark-comments/blob/main/LICENSE).
