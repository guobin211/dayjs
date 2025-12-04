# dayjs

[![Crates.io](https://img.shields.io/crates/v/dayjs.svg)](https://crates.io/crates/dayjs)
[![Documentation](https://docs.rs/dayjs/badge.svg)](https://docs.rs/dayjs)
[![License](https://img.shields.io/crates/l/dayjs.svg)](LICENSE)

A Rust library providing a JavaScript [Day.js](https://day.js.org/)-like API for date and time manipulation, built on top of `chrono`.

## Features

- 🚀 **Simple API** - JavaScript Day.js-inspired interface for Rust developers
- 🌍 **Timezone Support** - Handle different timezone formats (offset, city names, numeric)
- 📅 **Multiple Parsing Formats** - Support for RFC3339, RFC2822, ISO 8601, and more
- ⛓️ **Chainable Operations** - Add/subtract time units with method chaining
- 🔒 **Type Safety** - Leverages Rust's type system for safe time operations
- ⚡ **Zero-cost Abstractions** - Built on `chrono` for high performance

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
dayjs = "^0.1"
```

## Quick Start

```rust
use dayjs::{dayjs, from_str, DisplayTime, OperationTime};

fn main() {
    // Current date and time
    let now = dayjs();
    println!("Now: {}", now);

    // Parse from string
    let mut date = from_str("2025-01-25T10:30:45Z").unwrap();

    // Add 30 days
    date.add_days(30);

    // Format output
    println!("ISO: {}", date.to_iso());
    println!("Custom: {}", date.format("%Y-%m-%d %H:%M:%S"));
}
```

## API Reference

### Parse

```rust
use dayjs::{from_str, from_int64, from_ymd, from_ymdhms, from_array, from_format};

// From string (supports multiple formats)
let d1 = from_str("2025-01-25T10:30:45Z").unwrap();
let d2 = from_str("2025-01-25 10:30:45").unwrap();
let d3 = from_str("2025/01/25").unwrap();

// From Unix timestamp
let d4 = from_int64(1643164800).unwrap();      // seconds (10 digits)
let d5 = from_int64(1643164800000).unwrap();   // milliseconds (13 digits)

// From components
let d6 = from_ymd(2025, 1, 25).unwrap();
let d7 = from_ymdhms(2025, 1, 25, 10, 30, 45).unwrap();

// From array [year, month(0-11), day, hour, minute, second, ms]
let d8 = from_array(&[2025, 0, 25, 10, 30, 45]).unwrap();

// With custom format
let d9 = from_format("25-01-2025", "%d-%m-%Y").unwrap();
```

### Get + Set

```rust
use dayjs::dayjs;

let mut d = dayjs();

// Getters
d.year();           // e.g., 2025
d.month();          // 0-11 (JavaScript style)
d.date();           // 1-31 (day of month)
d.day();            // Weekday enum
d.hour();           // 0-23
d.minute();         // 0-59
d.second();         // Unix timestamp in seconds
d.millisecond();    // Unix timestamp in milliseconds

// Setters
d.set_year(2026);
d.set_month(6);     // July (0-indexed)
d.set_date(15);
d.set_hour(12);
d.set_minute(30);
d.set_second(0);
```

### Manipulate

```rust
use dayjs::{dayjs, OperationTime};

let mut d = dayjs();

// Add time
d.add_years(1);
d.add_months(2);
d.add_weeks(3);
d.add_days(4);
d.add_hours(5);
d.add_minutes(6);
d.add_seconds(7);
d.add_milliseconds(8);

// Subtract time
d.subtract_years(1);
d.subtract_months(2);
d.subtract_days(3);

// Start of / End of time unit
let start_of_month = d.start_of("month");
let end_of_year = d.end_of("year");
```

### Display

```rust
use dayjs::{dayjs, DisplayTime};

let d = dayjs();

// Custom format
d.format("%Y-%m-%d %H:%M:%S");

// Built-in formats
d.to_iso();     // "2025-01-25T10:30:45.000Z"
d.to_utc();     // "2025-01-25 10:30:45 +00:00"
d.to_gmt();     // "Sat, 25 Jan 2025 10:30:45 GMT"
d.to_array();   // "[ 2025, 0, 25, 10, 30, 45, 0 ]"
```

### Query

```rust
use dayjs::{from_str, QueryTime, Unit};

let d1 = from_str("2025-01-25").unwrap();
let d2 = from_str("2025-01-26").unwrap();
let d3 = from_str("2025-01-27").unwrap();

// Comparison
d1.is_before(&d2);                  // true
d1.is_after(&d2);                   // false
d1.is_same(&d2);                    // false
d1.is_same_unit(&d2, Unit::Month);  // true (same month)

// Inclusive comparison
d1.is_same_or_before(&d2);          // true
d1.is_same_or_after(&d2);           // false

// Range check
d2.is_between(&d1, &d3);            // true
```

### Diff

```rust
use dayjs::{from_str, DiffTime, Unit};

let d1 = from_str("2025-01-25").unwrap();
let d2 = from_str("2025-02-25").unwrap();

// Difference by unit
d2.diff(&d1, Unit::Day);    // 31
d2.diff(&d1, Unit::Month);  // 1

// Convenience methods
d2.diff_days(&d1);          // 31
d2.diff_hours(&d1);         // 744
d2.diff_minutes(&d1);       // 44640
```

### Utilities

```rust
use dayjs::{dayjs, from_str, min, max};

let d = dayjs();

// Date utilities
d.days_in_month();      // Number of days in current month
d.is_leap_year();       // Check if leap year
d.is_valid();           // Check if valid date

// Timestamps
d.unix();               // Unix timestamp (seconds)
d.value_of();           // Milliseconds since epoch

// Clone
let d2 = d.clone_dayjs();

// Min / Max
let d1 = from_str("2025-01-01").unwrap();
let d2 = from_str("2025-12-31").unwrap();
let earliest = min(&d1, &d2);
let latest = max(&d1, &d2);
```

## Supported Parse Formats

| Format | Example |
|--------|---------|
| RFC 3339 | `2025-01-25T10:30:45Z` |
| RFC 2822 | `Sat, 25 Jan 2025 10:30:45 +0000` |
| ISO 8601 | `2025-01-25T10:30:45+08:00` |
| Date Time | `2025-01-25 10:30:45` |
| Date Only | `2025-01-25`, `2025/01/25` |
| UTC Suffix | `2025-01-25 10:30:45 UTC` |

## License

[MIT](LICENSE)
