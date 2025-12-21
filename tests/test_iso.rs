#[cfg(test)]
mod tests {
    use chrono::Utc;
    use dayjs::{DisplayTime, TimeZone, dayjs};

    #[test]
    fn test_ios() {
        let now = dayjs::now();
        println!("{:?}", now.to_string());
        let end = now.end_of("day").to_iso();
        println!("{end}");
    }

    #[test]
    fn test_timezone() {
        // 本地时区
        let tz = TimeZone::current();
        println!("{:?}", tz);

        // 0时区，时间
        let now = Utc::now();
        println!("{:?}", now);
    }

    #[test]
    fn test_format() {
        let now = dayjs::now();
        let default = now.to_string();
        let utc = now.to_utc();
        let gmt = now.to_gmt();
        let iso = now.to_iso();
        println!("to_string(): {default}");
        println!("to_utc(): {utc}");
        println!("to_gmt(): {gmt}");
        println!("to_iso(): {iso}");
    }
}
