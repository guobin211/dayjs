//! # dayjs: dayjs api for Rust
//!
//! dayjs provides a simple and efficient way to work with date and time in Rust, inspired by the popular JavaScript library.
//!
use chrono::{
    DateTime, Datelike, FixedOffset, Local, Offset, TimeZone as CTimeZone, Timelike, Utc, Weekday,
};
use std::fmt::{Display, Formatter};

/// re-export chrono
pub use chrono;

/// TimeZone enum for representing different time zone formats.
#[derive(Clone, Debug, PartialEq)]
pub enum TimeZone {
    // 时区偏移, 如: "+08:00"
    TimeZoneTime(String),
    // 时区城市, 如: "Asia/Shanghai"
    TimeZoneCity(String),
    // 时区编号, -12 ~ +12
    TimeZoneNumber(i8),
}

impl TimeZone {
    /// Get the current time zone as a string.
    pub fn current() -> Self {
        let offset = Local::now().offset().fix();
        TimeZone::TimeZoneTime(offset.to_string())
    }
}

/// Dayjs struct representing a date and time with a time zone.
#[derive(Clone, Debug, PartialEq)]
pub struct Dayjs {
    /// Time zone information
    pub(crate) tz: TimeZone,
    /// UTC time
    pub(crate) time: DateTime<Utc>,
}

impl Default for Dayjs {
    fn default() -> Self {
        Dayjs {
            tz: TimeZone::current(),
            time: Utc::now(),
        }
    }
}

/// UTC Timestamp, eg: 143164800
pub fn timestamp() -> i64 {
    Utc::now().timestamp()
}

/// Create a new Dayjs instance with the current time.
pub fn dayjs() -> Dayjs {
    Dayjs::default()
}

/// Create a new Dayjs instance with the current time.
pub fn now() -> Dayjs {
    Dayjs::default()
}

/// Get Dayjs instance from a string representation of date time.
pub fn from_str(s: &str) -> Result<Dayjs, String> {
    let time: DateTime<Utc> =
        parse_date_time(s).ok_or_else(|| format!("Failed to parse date time from string: {s}"))?;
    Ok(Dayjs {
        time,
        ..Default::default()
    })
}

/// Get Dayjs instance from an integer timestamp.
pub fn from_int64(n: i64) -> Result<Dayjs, String> {
    let len = format!("{n}").len();
    match len {
        10 => {
            let r = Utc.timestamp_opt(n, 0);
            let r = r
                .single()
                .ok_or_else(|| format!("{n} is not a valid timestamp"))?;
            Ok(Dayjs {
                time: r,
                ..Default::default()
            })
        }
        13 => {
            let r = Utc.timestamp_millis_opt(n);
            let r = r
                .single()
                .ok_or_else(|| format!("{n} is not a valid timestamp"))?;
            Ok(Dayjs {
                time: r,
                ..Default::default()
            })
        }
        _ => Err(format!("{n} is not a safe time number"))?,
    }
}

/// Get the current time zone of the Dayjs instance.
pub fn from_timezone(tz: TimeZone) -> Dayjs {
    Dayjs {
        tz,
        time: Utc::now(),
    }
}

/// 解析日期时间字符串，支持 ISO 8601 格式（带时区偏移和 'Z' 后缀）以及 UTC 时间
///
/// # 参数
/// - `s`: 待解析的日期时间字符串
///
/// # 返回值
/// 解析成功返回 `DateTime<Utc>`，失败返回 `None`
pub fn parse_date_time(s: &str) -> Option<DateTime<Utc>> {
    if s.ends_with("UTC") || s.ends_with("utc") {
        let s = s.replace("UTC", "").replace("utc", "");
        let s = s.trim_end();
        let s = format!("{} {}", s, "+00:00");
        if let Ok(dt) = DateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S %:z") {
            return Some(dt.with_timezone(&Utc));
        }
    }
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return Some(dt.with_timezone(&Utc));
    }
    if let Ok(dt) = DateTime::parse_from_rfc2822(s) {
        return Some(dt.with_timezone(&Utc));
    }
    if let Some(offset) = FixedOffset::east_opt(0) {
        if let Ok(dt) = DateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S%.f %z") {
            return Some(dt.with_timezone(&offset).with_timezone(&Utc));
        }
        if let Ok(dt) = DateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.fZ") {
            return Some(dt.with_timezone(&offset).with_timezone(&Utc));
        }
        if let Ok(dt) = DateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S %:z") {
            return Some(dt.with_timezone(&offset).with_timezone(&Utc));
        }
        let s = format!("{} {}", s, "+00:00");
        if let Ok(dt) = DateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S %:z") {
            return Some(dt.with_timezone(&offset).with_timezone(&Utc));
        }
    }
    None
}

impl Display for Dayjs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = self.time.to_utc();
        let v = s.to_string();
        write!(f, "{v}")
    }
}

impl Dayjs {
    /// Format the date time according to the given template.
    ///
    /// # Parameters
    /// - `template`: %Y-%m-%d %H:%M:%S
    ///
    /// # Examples
    /// ```
    /// let now = dayjs::dayjs();
    /// let formatted = now.format("%Y-%m-%d %H:%M:%S");
    /// println!("{}", formatted);
    /// // 2025-03-25 17:21:47
    /// ```
    ///
    pub fn format(&self, template: &str) -> String {
        self.time.format(template).to_string()
    }

    /// Set the time zone for the Dayjs instance.
    pub fn set_timezone(&mut self, tz: TimeZone) {
        self.tz = tz;
    }

    /// Get the current time zone of the Dayjs instance.
    pub fn get_timezone(&self) -> &TimeZone {
        &self.tz
    }

    /// Get the current time in UTC.
    pub fn get_timestamp(&self) -> i64 {
        self.time.timestamp()
    }

    /// Get the current time in secend.
    pub fn timestap(&self) -> i64 {
        self.time.timestamp()
    }

    /// Get the current time in milliseconds since the Unix epoch.
    pub fn millisecond(&self) -> i64 {
        self.time.timestamp_millis()
    }

    /// Get the current time in seconds since the Unix epoch.
    pub fn second(&self) -> i64 {
        self.time.timestamp()
    }

    /// Get the current time in nanoseconds since the Unix epoch.
    pub fn minute(&self) -> u32 {
        self.time.minute()
    }

    /// Get the current time in hours since the Unix epoch.
    pub fn hour(&self) -> u32 {
        self.time.hour()
    }

    /// Get the date of month 1 ~ 31
    pub fn date(&self) -> u32 {
        self.time.day()
    }

    /// Get the week number 1 ~ 7
    pub fn day(&self) -> Weekday {
        self.time.weekday()
    }

    /// Get the month number 1 ~ 366
    pub fn day_of_year(&self) -> u32 {
        self.time.ordinal()
    }

    /// Get the week number 1 ~ 53
    pub fn week_of_year(&self) -> u32 {
        self.time.iso_week().week()
    }

    /// Get the month number 1 ~ 12
    pub fn month_of_year(&self) -> u32 {
        self.time.month()
    }
}

/// Trait for displaying time in various formats.
pub trait DisplayTime {
    /// Fromats to array string. [ 2019, 0, 25, 0, 0, 0, 0 ]
    fn to_array(&self) -> String;

    /// Fromats to iso string. "2019-01-25T02:00:00.000Z"
    fn to_iso(&self) -> String;

    /// Fromats to utc string. "2019-01-25 00:00:00 +00:00"
    fn to_utc(&self) -> String;

    /// Fromats to gmt string. "Fri, 25 Jan 2019 00:00:00 GMT"
    fn to_gmt(&self) -> String;

    /// Converts the time to a timestamp in seconds.
    fn to_timestamp(&self) -> i64;
}

impl DisplayTime for Dayjs {
    fn to_array(&self) -> String {
        let dt = self.time;
        format!(
            "[ {}, {}, {}, {}, {}, {}, {} ]",
            dt.year(),
            dt.month0(),
            dt.day0(),
            dt.hour(),
            dt.minute(),
            dt.second(),
            dt.nanosecond() / 1_000_000
        )
    }

    fn to_iso(&self) -> String {
        let dt = self.time;
        format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}Z",
            dt.year(),
            dt.month(),
            dt.day(),
            dt.hour(),
            dt.minute(),
            dt.second(),
            dt.nanosecond() / 1_000_000
        )
    }

    fn to_utc(&self) -> String {
        let dt = self.time;
        format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02} +00:00",
            dt.year(),
            dt.month(),
            dt.day(),
            dt.hour(),
            dt.minute(),
            dt.second()
        )
    }

    fn to_gmt(&self) -> String {
        let dt = self.time;
        format!(
            "{}, {:02} {} {:04} {:02}:{:02}:{:02} GMT",
            dt.weekday(),
            dt.day(),
            dt.format("%b"),
            dt.year(),
            dt.hour(),
            dt.minute(),
            dt.second()
        )
    }

    fn to_timestamp(&self) -> i64 {
        let dt = self.time;
        dt.timestamp()
    }
}

/// Trait for operations on time, such as adding or subtracting durations.
pub trait OperationTime {
    /// Add a duration to the current time.
    fn add(&mut self, timestamp: i32);

    /// Add a duration to the current time in year.
    fn add_years(&mut self, years: i32);

    /// Add a duration to the current time in month.
    fn add_months(&mut self, months: i32);

    /// Add a duration to the current time in week.
    fn add_weeks(&mut self, weeks: i32);

    /// Add a duration to the current time in day.
    fn add_days(&mut self, days: i32);

    /// Add a duration to the current time in hours.
    fn add_hours(&mut self, hours: i32);

    /// Add a duration to the current time in minutes.
    fn add_minutes(&mut self, minutes: i32);

    /// Add a duration to the current time in seconds.
    fn add_seconds(&mut self, seconds: i32);

    /// Add a duration to the current time in milliseconds.
    fn add_milliseconds(&mut self, milliseconds: i32);

    /// Subtract a duration from the current time.
    fn subtract(&mut self, timestamp: i32);

    /// Subtract a duration from the current time in year.
    fn subtract_years(&mut self, years: i32);

    /// Subtract a duration from the current time in month.
    fn subtract_months(&mut self, months: i32);

    /// Subtract a duration from the current time in week.
    fn subtract_weeks(&mut self, weeks: i32);

    /// Subtract a duration from the current time in day.
    fn subtract_days(&mut self, days: i32);

    /// Subtract a duration from the current time in hours.
    fn subtract_hours(&mut self, hours: i32);

    /// Subtract a duration from the current time in minutes.
    fn subtract_minutes(&mut self, minutes: i32);

    /// Subtract a duration from the current time in seconds.
    fn subtract_seconds(&mut self, seconds: i32);

    /// Subtract a duration from the current time in milliseconds.
    fn subtract_milliseconds(&mut self, milliseconds: i32);
}

impl OperationTime for Dayjs {
    fn add(&mut self, timestamp: i32) {
        let dt = self.time + chrono::Duration::seconds(timestamp as i64);
        self.time = dt;
    }

    fn add_years(&mut self, years: i32) {
        let dt = self.time + chrono::Duration::days((years * 365) as i64);
        self.time = dt;
    }

    fn add_months(&mut self, months: i32) {
        let mut dt = self.time;
        for _ in 0..months {
            dt = dt.with_month(dt.month() + 1).unwrap_or(dt);
        }
        self.time = dt;
    }

    fn add_weeks(&mut self, weeks: i32) {
        let dt = self.time + chrono::Duration::weeks(weeks as i64);
        self.time = dt;
    }

    fn add_days(&mut self, days: i32) {
        let dt = self.time + chrono::Duration::days(days as i64);
        self.time = dt;
    }

    fn add_hours(&mut self, hours: i32) {
        let dt = self.time + chrono::Duration::hours(hours as i64);
        self.time = dt;
    }

    fn add_minutes(&mut self, minutes: i32) {
        let dt = self.time + chrono::Duration::minutes(minutes as i64);
        self.time = dt;
    }

    fn add_seconds(&mut self, seconds: i32) {
        let dt = self.time + chrono::Duration::seconds(seconds as i64);
        self.time = dt;
    }

    fn add_milliseconds(&mut self, milliseconds: i32) {
        let dt = self.time + chrono::Duration::milliseconds(milliseconds as i64);
        self.time = dt;
    }

    fn subtract(&mut self, timestamp: i32) {
        let dt = self.time - chrono::Duration::seconds(timestamp as i64);
        self.time = dt;
    }

    fn subtract_years(&mut self, years: i32) {
        let dt = self.time - chrono::Duration::days(years as i64);
        self.time = dt;
    }

    fn subtract_months(&mut self, months: i32) {
        let mut dt = self.time;
        for _ in 0..months {
            dt = dt.with_month(dt.month() - 1).unwrap_or(dt);
        }
        self.time = dt;
    }

    fn subtract_weeks(&mut self, weeks: i32) {
        let dt = self.time - chrono::Duration::weeks(weeks as i64);
        self.time = dt;
    }

    fn subtract_days(&mut self, days: i32) {
        let dt = self.time - chrono::Duration::days(days as i64);
        self.time = dt;
    }

    fn subtract_hours(&mut self, hours: i32) {
        let dt = self.time - chrono::Duration::hours(hours as i64);
        self.time = dt;
    }

    fn subtract_minutes(&mut self, minutes: i32) {
        let dt = self.time - chrono::Duration::minutes(minutes as i64);
        self.time = dt;
    }

    fn subtract_seconds(&mut self, seconds: i32) {
        let dt = self.time - chrono::Duration::seconds(seconds as i64);
        self.time = dt;
    }

    fn subtract_milliseconds(&mut self, milliseconds: i32) {
        let dt = self.time - chrono::Duration::milliseconds(milliseconds as i64);
        self.time = dt;
    }
}
