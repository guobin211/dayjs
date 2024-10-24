# dayjs

dayjs for rust

## Usage

```rust
fn main() {
    // default
    let now = dayjs::dayjs();
    // timestamp
    let date3 = dayjs::from_timestamp(1729746657208);
    // GMT
    let date1 = dayjs::from_str("Thu, 24 Oct 2024 05:12:08 GMT");
    // ISO
    let date2 = dayjs::from_str("2024-10-24T05:12:15.395Z");
}
```
