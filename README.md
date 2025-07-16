# dayjs

A Rust library providing a JavaScript Day.js-like API for date and time manipulation, built on top of `chrono`.

## Features

- **Simple API**: JavaScript Day.js-inspired interface for Rust developers
- **Timezone Support**: Handle different timezone formats (offset, city names, numeric)
- **Multiple Parsing Formats**: Support for RFC3339, RFC2822, UTC suffix, and more
- **Chainable Operations**: Add/subtract time units with method chaining
- **Type Safety**: Leverages Rust's type system for safe time operations
- **Zero-cost Abstractions**: Built on `chrono` for high performance

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
dayjs = "^0.1"
```

## Usage

```rust

fn main() {
    // current date and time
    let now = dayjs::now();

    // Create and manipulate dates
    let mut date = dayjs::from_str("2025-01-25T10:30:45Z").unwrap();

    // Add 30 days
    date.add_days(30);

    // Format output
    println!("ISO: {}", date.to_iso());
    println!("Custom: {}", date.format("%Y年%m月%d日 %H:%M:%S"));
}
```
