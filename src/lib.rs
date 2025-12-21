/// re-export chrono
pub use chrono;
use chrono::{
    DateTime, Datelike, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, Offset,
    TimeZone as CTimeZone, Timelike, Utc, Weekday,
};
use serde_json::{Value, json};
use std::fmt::{Display, Formatter};

/// TimeZone enum for representing different time zone formats.
///
/// It can represent:
/// - Time zone offset as a string (e.g., "+08:00")
/// - Time zone city as a string (e.g., "Asia/Shanghai")
/// - Time zone number as an integer (-12 to +12)
///
/// # Examples
///
/// ```
/// use dayjs::TimeZone;
///
/// let tz_offset = TimeZone::TimeZoneTime("+08:00".to_string());
///
/// let tz_city = TimeZone::TimeZoneCity("Asia/Shanghai".to_string());
///
/// let tz_number = TimeZone::TimeZoneNumber(8);
///
#[derive(Clone, Debug, PartialEq)]
pub enum TimeZone {
    // offset, 如: "+08:00"
    TimeZoneTime(String),
    // city, 如: "Asia/Shanghai"
    TimeZoneCity(String),
    // order, -12 ~ +12
    TimeZoneNumber(i8),
}

impl TimeZone {
    /// Get the current time zone as a string.
    pub fn current() -> Self {
        let offset = Local::now().offset().fix();
        TimeZone::TimeZoneTime(offset.to_string())
    }

    /// Parse timezone from string
    ///
    /// Attempts to parse a string into a TimeZone enum:
    /// - If it starts with '+' or '-', treats it as TimeZoneTime (offset)
    /// - If it's a valid number between -12 and 12, treats it as TimeZoneNumber
    /// - Otherwise, treats it as TimeZoneCity
    pub fn from_string(s: String) -> Self {
        let s = s.trim();

        // Check if it's an offset format (starts with + or -)
        if s.starts_with('+') || s.starts_with('-') {
            return TimeZone::TimeZoneTime(s.to_string());
        }

        // Try to parse as number
        if let Ok(num) = s.parse::<i8>()
            && (-12..=14).contains(&num)
        {
            return TimeZone::TimeZoneNumber(num);
        }

        // Default to city format
        TimeZone::TimeZoneCity(s.to_string())
    }
}

impl Display for TimeZone {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeZone::TimeZoneTime(s) => write!(f, "{}", s),
            TimeZone::TimeZoneCity(s) => write!(f, "{}", s),
            TimeZone::TimeZoneNumber(n) => write!(f, "{}", n),
        }
    }
}

/// Dayjs struct representing a date and time with a time zone.
///
/// It contains:
/// - `tz`: The time zone information as a `TimeZone` enum.
/// - `time`: The UTC time as a `DateTime<Utc>`.
///
/// # Examples
/// ```
/// use dayjs::{dayjs, Dayjs, TimeZone};
/// let now = dayjs();
/// ```
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

impl Dayjs {
    /// Convert Dayjs to JSON object
    ///
    /// # Returns
    /// Returns a JSON object with timezone and time information
    ///
    /// # Examples
    /// ```
    /// use dayjs::dayjs;
    /// let d = dayjs();
    /// let obj = d.to_object();
    /// println!("{}", obj);
    /// ```
    pub fn to_object(&self) -> Value {
        json!({
            "tz": self.tz.to_string(),
            "time": self.time.to_rfc3339(),
        })
    }

    /// Create Dayjs from JSON object
    ///
    /// # Parameters
    /// - `obj`: JSON object containing "tz" and "time" fields
    ///
    /// # Returns
    /// Returns a Result with Dayjs on success, error message on failure
    ///
    /// # Examples
    /// ```
    /// use dayjs::Dayjs;
    /// use serde_json::json;
    ///
    /// let obj = json!({
    ///     "tz": "+08:00",
    ///     "time": "2025-12-21T14:30:00Z"
    /// });
    /// let d = Dayjs::from_object(obj).unwrap();
    /// ```
    pub fn from_object(obj: Value) -> Result<Self, String> {
        // Extract timezone
        let tz_str = obj["tz"]
            .as_str()
            .ok_or("Missing or invalid 'tz' field")?
            .to_string();

        // Extract time string
        let time_str = obj["time"]
            .as_str()
            .ok_or("Missing or invalid 'time' field")?;

        // Parse timezone
        let tz = TimeZone::from_string(tz_str);

        // Parse time - try multiple formats
        let time = if let Ok(dt) = DateTime::parse_from_rfc3339(time_str) {
            dt.with_timezone(&Utc)
        } else if let Some(dt) = parse_date_time(time_str) {
            dt
        } else {
            return Err(format!("Failed to parse time: {}", time_str));
        };

        Ok(Dayjs { tz, time })
    }
}

/// UTC Timestamp, eg: 143164800
///
/// # Examples
/// ```
/// use dayjs::timestamp;
/// let ts = timestamp();
/// println!("Current UTC timestamp: {}", ts);
/// ```
///
pub fn timestamp() -> i64 {
    Utc::now().timestamp()
}

/// Create a new Dayjs instance with the current time.
///
/// # Examples
/// ```
/// use dayjs::dayjs;
/// let now = dayjs();
/// println!("Current time: {}", now);
/// ```
pub fn dayjs() -> Dayjs {
    Dayjs::default()
}

/// Create a new Dayjs instance with the current time.
/// This is an alias for `dayjs()`.
/// # Examples
/// ```
/// use dayjs::now;
/// let current_time = now();
/// println!("Current time: {}", current_time);
/// ```
pub fn now() -> Dayjs {
    Dayjs::default()
}

/// Get Dayjs instance from a string representation of date time.
///
/// # Parameters
/// - `s`: A string representing the date time, which can be in various formats such as ISO 8601, RFC 3339, or RFC 2822.
/// # Returns
/// - `Ok(Dayjs)`: If the string is successfully parsed into a `DateTime<Utc>`.
/// - `Err(String)`: If the string cannot be parsed, an error message is returned.
///
/// # Examples
/// ```
/// use dayjs::from_str;
/// let date_str = "2023-10-01T12:00:00Z";
/// let dayjs_instance = from_str(date_str);
/// match dayjs_instance {
///     Ok(dayjs) => println!("Parsed Dayjs: {}", dayjs),
///     Err(e) => println!("Error parsing date: {}", e),
/// }
/// ```
pub fn from_str(s: &str) -> Result<Dayjs, String> {
    let time: DateTime<Utc> =
        parse_date_time(s).ok_or_else(|| format!("Failed to parse date time from string: {s}"))?;
    Ok(Dayjs {
        time,
        ..Default::default()
    })
}

/// Get Dayjs instance from an integer timestamp.
/// # Parameters
/// - `n`: An integer representing the timestamp, which can be in seconds (10 digits) or milliseconds (13 digits).
/// # Returns
/// - `Ok(Dayjs)`: If the integer is successfully converted to a `DateTime<Utc>`.
/// - `Err(String)`: If the integer is not a valid timestamp or does not match the expected length (10 or 13 digits).
///
/// # Examples
/// ```
/// use dayjs::from_int64;
/// let timestamp = 1633072800; // Example timestamp in seconds
/// let dayjs_instance = from_int64(timestamp);
/// match dayjs_instance {
///     Ok(dayjs) => println!("Parsed Dayjs: {}", dayjs),
///     Err(e) => println!("Error parsing timestamp: {}", e),
/// }
/// ```
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
///
/// # Parameters
/// - `tz`: A `TimeZone` enum representing the desired time zone.
/// # Returns
/// A `Dayjs` instance with the specified time zone and the current UTC time.
///
/// # Examples
/// ```
/// use dayjs::{from_timezone, TimeZone};
/// let tz = TimeZone::TimeZoneTime("+08:00".to_string());
/// let dayjs_instance = from_timezone(tz);
/// println!("Current time in specified timezone: {}", dayjs_instance);
/// ```
pub fn from_timezone(tz: TimeZone) -> Dayjs {
    Dayjs {
        tz,
        time: Utc::now(),
    }
}

/// Parse timezone offset string like "+08:00", "-05:00" into FixedOffset
///
/// # Parameters
/// - `offset_str`: The offset string to parse (e.g., "+08:00", "-05:00")
///
/// # Returns
/// Returns `FixedOffset` on successful parsing, error otherwise
fn parse_offset(offset_str: &str) -> Result<FixedOffset, String> {
    let offset_str = offset_str.trim();

    // Parse format: "+08:00" or "-05:00"
    if offset_str.len() < 5 {
        return Err("Invalid offset format".to_string());
    }

    let sign = match &offset_str[0..1] {
        "+" => 1,
        "-" => -1,
        _ => return Err("Offset must start with + or -".to_string()),
    };

    // Try to parse hours and minutes
    let parts: Vec<&str> = offset_str[1..].split(':').collect();
    if parts.len() != 2 {
        return Err("Invalid offset format, expected HH:MM".to_string());
    }

    let hours: i32 = parts[0].parse().map_err(|_| "Invalid hours")?;
    let minutes: i32 = parts[1].parse().map_err(|_| "Invalid minutes")?;

    let total_seconds = sign * (hours * 3600 + minutes * 60);

    FixedOffset::east_opt(total_seconds).ok_or_else(|| "Offset out of valid range".to_string())
}

/// Parse date time string, supports ISO 8601 format (with timezone offset and 'Z' suffix) and UTC time
///
/// # Parameters
/// - `s`: The date time string to be parsed
///
/// # Returns
/// Returns `DateTime<Utc>` on successful parsing, `None` on failure
pub fn parse_date_time(s: &str) -> Option<DateTime<Utc>> {
    let s = s.trim();

    // Handle UTC suffix
    if s.ends_with("UTC") || s.ends_with("utc") {
        let s = s.replace("UTC", "").replace("utc", "");
        let s = s.trim_end();
        let s = format!("{} {}", s, "+00:00");
        if let Ok(dt) = DateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S %:z") {
            return Some(dt.with_timezone(&Utc));
        }
    }

    // Try RFC3339 format
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return Some(dt.with_timezone(&Utc));
    }

    // Try RFC2822 format
    if let Ok(dt) = DateTime::parse_from_rfc2822(s) {
        return Some(dt.with_timezone(&Utc));
    }

    // Try various formats with timezone
    let formats_with_tz = [
        "%Y-%m-%d %H:%M:%S%.f %z",
        "%Y-%m-%dT%H:%M:%S%.fZ",
        "%Y-%m-%d %H:%M:%S %:z",
        "%Y-%m-%dT%H:%M:%S%:z",
        "%Y-%m-%dT%H:%M:%S%.f%:z",
        "%Y/%m/%d %H:%M:%S %:z",
    ];

    for fmt in &formats_with_tz {
        if let Ok(dt) = DateTime::parse_from_str(s, fmt) {
            return Some(dt.with_timezone(&Utc));
        }
    }

    // Try formats without timezone (assume UTC)
    let formats_without_tz = [
        "%Y-%m-%d %H:%M:%S%.f",
        "%Y-%m-%d %H:%M:%S",
        "%Y-%m-%dT%H:%M:%S%.f",
        "%Y-%m-%dT%H:%M:%S",
        "%Y/%m/%d %H:%M:%S",
        "%Y/%m/%d",
        "%Y-%m-%d",
    ];

    for fmt in &formats_without_tz {
        if let Ok(ndt) = NaiveDateTime::parse_from_str(s, fmt) {
            return Some(ndt.and_utc());
        }
    }

    // Try date only formats
    let date_formats = ["%Y-%m-%d", "%Y/%m/%d", "%d-%m-%Y", "%d/%m/%Y"];

    for fmt in &date_formats {
        if let Ok(nd) = NaiveDate::parse_from_str(s, fmt) {
            let ndt = nd.and_time(NaiveTime::from_hms_opt(0, 0, 0)?);
            return Some(ndt.and_utc());
        }
    }

    None
}

/// Display implementation for Dayjs
/// Displays the UTC time in ISO 8601 format (e.g., 2025-12-21T14:29:42.009427Z)
impl Display for Dayjs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.time.format("%Y-%m-%dT%H:%M:%S%.6fZ"))
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

    /// Get the current time in second.
    pub fn timestamp(&self) -> i64 {
        self.time.timestamp()
    }

    /// Get the millisecond (0-999)
    pub fn millisecond(&self) -> u32 {
        self.time.timestamp_subsec_millis()
    }

    /// Get the second (0-59)
    pub fn second(&self) -> u32 {
        self.time.second()
    }

    /// Get the minute (0-59)
    pub fn minute(&self) -> u32 {
        self.time.minute()
    }

    /// Get the hour (0-23)
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

    /// Get the year
    pub fn year(&self) -> i32 {
        self.time.year()
    }

    /// Get the month (0-11, JavaScript style)
    pub fn month(&self) -> u32 {
        self.time.month0()
    }

    /// Set the year
    pub fn set_year(&mut self, year: i32) {
        if let Some(dt) = self.time.with_year(year) {
            self.time = dt;
        }
    }

    /// Set the month (1-12)
    pub fn set_month(&mut self, month: u32) {
        if let Some(dt) = self.time.with_month(month) {
            self.time = dt;
        }
    }

    /// Set the day of month (1-31)
    pub fn set_date(&mut self, day: u32) {
        if let Some(dt) = self.time.with_day(day) {
            self.time = dt;
        }
    }

    /// Set the hour (0-23)
    pub fn set_hour(&mut self, hour: u32) {
        if let Some(dt) = self.time.with_hour(hour) {
            self.time = dt;
        }
    }

    /// Set the minute (0-59)
    pub fn set_minute(&mut self, minute: u32) {
        if let Some(dt) = self.time.with_minute(minute) {
            self.time = dt;
        }
    }

    /// Set the second (0-59)
    pub fn set_second(&mut self, second: u32) {
        if let Some(dt) = self.time.with_second(second) {
            self.time = dt;
        }
    }

    /// Set the millisecond (0-999)
    pub fn set_millisecond(&mut self, ms: u32) {
        if let Some(dt) = self.time.with_nanosecond(ms * 1_000_000) {
            self.time = dt;
        }
    }

    /// Get start of a unit of time
    pub fn start_of(&self, unit: &str) -> Dayjs {
        let mut result = self.clone();

        // Convert to local timezone for day-based operations
        let local_time = self.time.with_timezone(&Local);

        match unit.to_lowercase().as_str() {
            "year" => {
                let adjusted = local_time
                    .with_month(1)
                    .and_then(|t| t.with_day(1))
                    .and_then(|t| t.with_hour(0))
                    .and_then(|t| t.with_minute(0))
                    .and_then(|t| t.with_second(0))
                    .and_then(|t| t.with_nanosecond(0));
                if let Some(dt) = adjusted {
                    result.time = dt.with_timezone(&Utc);
                }
            }
            "month" => {
                let adjusted = local_time
                    .with_day(1)
                    .and_then(|t| t.with_hour(0))
                    .and_then(|t| t.with_minute(0))
                    .and_then(|t| t.with_second(0))
                    .and_then(|t| t.with_nanosecond(0));
                if let Some(dt) = adjusted {
                    result.time = dt.with_timezone(&Utc);
                }
            }
            "week" => {
                let weekday = local_time.weekday().num_days_from_sunday();
                let adjusted = (local_time - chrono::Duration::days(weekday as i64))
                    .with_hour(0)
                    .and_then(|t| t.with_minute(0))
                    .and_then(|t| t.with_second(0))
                    .and_then(|t| t.with_nanosecond(0));
                if let Some(dt) = adjusted {
                    result.time = dt.with_timezone(&Utc);
                }
            }
            "day" | "date" => {
                let adjusted = local_time
                    .with_hour(0)
                    .and_then(|t| t.with_minute(0))
                    .and_then(|t| t.with_second(0))
                    .and_then(|t| t.with_nanosecond(0));
                if let Some(dt) = adjusted {
                    result.time = dt.with_timezone(&Utc);
                }
            }
            "hour" => {
                result.time = result
                    .time
                    .with_minute(0)
                    .and_then(|t| t.with_second(0))
                    .and_then(|t| t.with_nanosecond(0))
                    .unwrap_or(result.time);
            }
            "minute" => {
                result.time = result
                    .time
                    .with_second(0)
                    .and_then(|t| t.with_nanosecond(0))
                    .unwrap_or(result.time);
            }
            "second" => {
                result.time = result.time.with_nanosecond(0).unwrap_or(result.time);
            }
            _ => {}
        }
        result
    }

    /// Get end of a unit of time
    pub fn end_of(&self, unit: &str) -> Dayjs {
        let mut result = self.clone();

        // Convert to local timezone for day-based operations
        let local_time = self.time.with_timezone(&Local);

        match unit.to_lowercase().as_str() {
            "year" => {
                let adjusted = local_time
                    .with_month(12)
                    .and_then(|t| t.with_day(31))
                    .and_then(|t| t.with_hour(23))
                    .and_then(|t| t.with_minute(59))
                    .and_then(|t| t.with_second(59))
                    .and_then(|t| t.with_nanosecond(999_999_999));
                if let Some(dt) = adjusted {
                    result.time = dt.with_timezone(&Utc);
                }
            }
            "month" => {
                let next_month = local_time.with_day(1).and_then(|t| {
                    if t.month() == 12 {
                        t.with_year(t.year() + 1).and_then(|t| t.with_month(1))
                    } else {
                        t.with_month(t.month() + 1)
                    }
                });
                if let Some(next) = next_month {
                    let adjusted = (next - chrono::Duration::days(1))
                        .with_hour(23)
                        .and_then(|t| t.with_minute(59))
                        .and_then(|t| t.with_second(59))
                        .and_then(|t| t.with_nanosecond(999_999_999));
                    if let Some(dt) = adjusted {
                        result.time = dt.with_timezone(&Utc);
                    }
                }
            }
            "week" => {
                let weekday = local_time.weekday().num_days_from_sunday();
                let days_to_saturday = 6 - weekday;
                let adjusted = (local_time + chrono::Duration::days(days_to_saturday as i64))
                    .with_hour(23)
                    .and_then(|t| t.with_minute(59))
                    .and_then(|t| t.with_second(59))
                    .and_then(|t| t.with_nanosecond(999_999_999));
                if let Some(dt) = adjusted {
                    result.time = dt.with_timezone(&Utc);
                }
            }
            "day" | "date" => {
                let adjusted = local_time
                    .with_hour(23)
                    .and_then(|t| t.with_minute(59))
                    .and_then(|t| t.with_second(59))
                    .and_then(|t| t.with_nanosecond(999_999_999));
                if let Some(dt) = adjusted {
                    result.time = dt.with_timezone(&Utc);
                }
            }
            "hour" => {
                result.time = result
                    .time
                    .with_minute(59)
                    .and_then(|t| t.with_second(59))
                    .and_then(|t| t.with_nanosecond(999_999_999))
                    .unwrap_or(result.time);
            }
            "minute" => {
                result.time = result
                    .time
                    .with_second(59)
                    .and_then(|t| t.with_nanosecond(999_999_999))
                    .unwrap_or(result.time);
            }
            "second" => {
                result.time = result
                    .time
                    .with_nanosecond(999_999_999)
                    .unwrap_or(result.time);
            }
            _ => {}
        }
        result
    }

    /// Clone the Dayjs instance
    pub fn clone_dayjs(&self) -> Dayjs {
        self.clone()
    }

    /// Get the number of days in the month
    pub fn days_in_month(&self) -> u32 {
        let year = self.time.year();
        let month = self.time.month();
        if month == 12 {
            NaiveDate::from_ymd_opt(year + 1, 1, 1)
        } else {
            NaiveDate::from_ymd_opt(year, month + 1, 1)
        }
        .and_then(|d| d.pred_opt())
        .map(|d| d.day())
        .unwrap_or(30)
    }

    /// Check if the year is a leap year
    pub fn is_leap_year(&self) -> bool {
        let year = self.time.year();
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    /// Convert to JavaScript Date valueOf (milliseconds since epoch)
    pub fn value_of(&self) -> i64 {
        self.time.timestamp_millis()
    }

    /// Convert to Unix timestamp (seconds since epoch)
    pub fn unix(&self) -> i64 {
        self.time.timestamp()
    }

    /// Check if the Dayjs object is valid
    pub fn is_valid(&self) -> bool {
        true
    }
}

/// Trait for displaying time in various formats.
pub trait DisplayTime {
    /// Formats to array string. [ 2019, 0, 25, 0, 0, 0, 0 ]
    fn to_array(&self) -> String;

    /// Formats to iso string. "2025-12-21T14:44:55.240Z"
    fn to_iso(&self) -> String;

    /// Formats to utc string. "2025-12-21T14:44:55.240434Z"
    fn to_utc(&self) -> String;

    /// Formats to gmt string. "Sun, 21 Dec 2025 14:44:55 GMT"
    fn to_gmt(&self) -> String;

    /// Converts the time to a timestamp in seconds.
    fn to_timestamp(&self) -> i64;

    /// Formats the date time to a local string.
    fn to_local(&self) -> String;
}

impl DisplayTime for Dayjs {
    /// Formats the date time to an array string.
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

    /// Formats the date time to an ISO 8601 string.
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

    /// Formats the date time to a UTC string.
    fn to_utc(&self) -> String {
        self.to_string()
    }

    /// Formats the date time to a GMT string.
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

    /// Converts the time to a timestamp in seconds.
    fn to_timestamp(&self) -> i64 {
        let dt = self.time;
        dt.timestamp()
    }

    /// Formats the date time to a local string.
    fn to_local(&self) -> String {
        // Convert UTC time to timezone specified in self.tz
        match &self.tz {
            TimeZone::TimeZoneTime(offset_str) => {
                // Parse offset string like "+08:00" or "-05:00"
                if let Ok(offset) = parse_offset(offset_str) {
                    let local_time = self.time.with_timezone(&offset);
                    format!("{}", local_time.format("%Y-%m-%dT%H:%M:%S%.6f%:z"))
                } else {
                    // Fallback to system local time if parsing fails
                    let local_time: DateTime<Local> = DateTime::from(self.time);
                    format!("{}", local_time.format("%Y-%m-%dT%H:%M:%S%.6f%:z"))
                }
            }
            TimeZone::TimeZoneNumber(hours) => {
                // Convert hours to seconds for FixedOffset
                let seconds = (*hours as i32) * 3600;
                if let Some(offset) = FixedOffset::east_opt(seconds) {
                    let local_time = self.time.with_timezone(&offset);
                    format!("{}", local_time.format("%Y-%m-%dT%H:%M:%S%.6f%:z"))
                } else {
                    // Fallback to UTC if offset is invalid
                    format!("{}", self.time.format("%Y-%m-%dT%H:%M:%S%.6fZ"))
                }
            }
            TimeZone::TimeZoneCity(_city) => {
                // For city-based timezones, fallback to system local time
                // Note: Full city timezone support requires chrono-tz crate
                let local_time: DateTime<Local> = DateTime::from(self.time);
                format!("{}", local_time.format("%Y-%m-%dT%H:%M:%S%.6f%:z"))
            }
        }
    }
}

/// Unit enum for time operations
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Unit {
    Year,
    Month,
    Week,
    Day,
    Hour,
    Minute,
    Second,
    Millisecond,
}

impl Unit {
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<Unit, String> {
        match s.to_lowercase().as_str() {
            "year" | "years" | "y" => Ok(Unit::Year),
            "month" | "months" => Ok(Unit::Month),
            "week" | "weeks" | "w" => Ok(Unit::Week),
            "day" | "days" | "d" => Ok(Unit::Day),
            "hour" | "hours" | "h" => Ok(Unit::Hour),
            "minute" | "minutes" | "m" => Ok(Unit::Minute),
            "second" | "seconds" | "s" => Ok(Unit::Second),
            "millisecond" | "milliseconds" | "ms" => Ok(Unit::Millisecond),
            _ => Err(format!("Unknown unit: {}", s)),
        }
    }
}

/// Trait for querying and comparing times
pub trait QueryTime {
    /// Check if this time is before another time
    fn is_before(&self, other: &Dayjs) -> bool;

    /// Check if this time is before another time with unit granularity
    fn is_before_unit(&self, other: &Dayjs, unit: Unit) -> bool;

    /// Check if this time is after another time
    fn is_after(&self, other: &Dayjs) -> bool;

    /// Check if this time is after another time with unit granularity
    fn is_after_unit(&self, other: &Dayjs, unit: Unit) -> bool;

    /// Check if this time is the same as another time
    fn is_same(&self, other: &Dayjs) -> bool;

    /// Check if this time is the same as another time with unit granularity
    fn is_same_unit(&self, other: &Dayjs, unit: Unit) -> bool;

    /// Check if this time is the same or before another time
    fn is_same_or_before(&self, other: &Dayjs) -> bool;

    /// Check if this time is the same or after another time
    fn is_same_or_after(&self, other: &Dayjs) -> bool;

    /// Check if this time is between two other times
    fn is_between(&self, start: &Dayjs, end: &Dayjs) -> bool;

    /// Check if this time is between two other times with unit granularity
    fn is_between_unit(&self, start: &Dayjs, end: &Dayjs, unit: Unit) -> bool;
}

impl QueryTime for Dayjs {
    fn is_before(&self, other: &Dayjs) -> bool {
        self.time < other.time
    }

    fn is_before_unit(&self, other: &Dayjs, unit: Unit) -> bool {
        let unit_str = match unit {
            Unit::Year => "year",
            Unit::Month => "month",
            Unit::Week => "week",
            Unit::Day => "day",
            Unit::Hour => "hour",
            Unit::Minute => "minute",
            Unit::Second => "second",
            Unit::Millisecond => return self.time < other.time,
        };
        self.start_of(unit_str).time < other.start_of(unit_str).time
    }

    fn is_after(&self, other: &Dayjs) -> bool {
        self.time > other.time
    }

    fn is_after_unit(&self, other: &Dayjs, unit: Unit) -> bool {
        let unit_str = match unit {
            Unit::Year => "year",
            Unit::Month => "month",
            Unit::Week => "week",
            Unit::Day => "day",
            Unit::Hour => "hour",
            Unit::Minute => "minute",
            Unit::Second => "second",
            Unit::Millisecond => return self.time > other.time,
        };
        self.start_of(unit_str).time > other.start_of(unit_str).time
    }

    fn is_same(&self, other: &Dayjs) -> bool {
        self.time == other.time
    }

    fn is_same_unit(&self, other: &Dayjs, unit: Unit) -> bool {
        let unit_str = match unit {
            Unit::Year => "year",
            Unit::Month => "month",
            Unit::Week => "week",
            Unit::Day => "day",
            Unit::Hour => "hour",
            Unit::Minute => "minute",
            Unit::Second => "second",
            Unit::Millisecond => return self.time == other.time,
        };
        self.start_of(unit_str).time == other.start_of(unit_str).time
    }

    fn is_same_or_before(&self, other: &Dayjs) -> bool {
        self.time <= other.time
    }

    fn is_same_or_after(&self, other: &Dayjs) -> bool {
        self.time >= other.time
    }

    fn is_between(&self, start: &Dayjs, end: &Dayjs) -> bool {
        self.time > start.time && self.time < end.time
    }

    fn is_between_unit(&self, start: &Dayjs, end: &Dayjs, unit: Unit) -> bool {
        let unit_str = match unit {
            Unit::Year => "year",
            Unit::Month => "month",
            Unit::Week => "week",
            Unit::Day => "day",
            Unit::Hour => "hour",
            Unit::Minute => "minute",
            Unit::Second => "second",
            Unit::Millisecond => return self.time > start.time && self.time < end.time,
        };
        let self_start = self.start_of(unit_str).time;
        let start_start = start.start_of(unit_str).time;
        let end_start = end.start_of(unit_str).time;
        self_start > start_start && self_start < end_start
    }
}

/// Trait for calculating differences between times
pub trait DiffTime {
    /// Get the difference between two times in the specified unit
    fn diff(&self, other: &Dayjs, unit: Unit) -> i64;

    /// Get the difference in milliseconds
    fn diff_milliseconds(&self, other: &Dayjs) -> i64;

    /// Get the difference in seconds
    fn diff_seconds(&self, other: &Dayjs) -> i64;

    /// Get the difference in minutes
    fn diff_minutes(&self, other: &Dayjs) -> i64;

    /// Get the difference in hours
    fn diff_hours(&self, other: &Dayjs) -> i64;

    /// Get the difference in days
    fn diff_days(&self, other: &Dayjs) -> i64;

    /// Get the difference in weeks
    fn diff_weeks(&self, other: &Dayjs) -> i64;

    /// Get the difference in months
    fn diff_months(&self, other: &Dayjs) -> i64;

    /// Get the difference in years
    fn diff_years(&self, other: &Dayjs) -> i64;
}

impl DiffTime for Dayjs {
    fn diff(&self, other: &Dayjs, unit: Unit) -> i64 {
        match unit {
            Unit::Year => self.diff_years(other),
            Unit::Month => self.diff_months(other),
            Unit::Week => self.diff_weeks(other),
            Unit::Day => self.diff_days(other),
            Unit::Hour => self.diff_hours(other),
            Unit::Minute => self.diff_minutes(other),
            Unit::Second => self.diff_seconds(other),
            Unit::Millisecond => self.diff_milliseconds(other),
        }
    }

    fn diff_milliseconds(&self, other: &Dayjs) -> i64 {
        self.time.timestamp_millis() - other.time.timestamp_millis()
    }

    fn diff_seconds(&self, other: &Dayjs) -> i64 {
        self.time.timestamp() - other.time.timestamp()
    }

    fn diff_minutes(&self, other: &Dayjs) -> i64 {
        self.diff_seconds(other) / 60
    }

    fn diff_hours(&self, other: &Dayjs) -> i64 {
        self.diff_seconds(other) / 3600
    }

    fn diff_days(&self, other: &Dayjs) -> i64 {
        self.diff_seconds(other) / 86400
    }

    fn diff_weeks(&self, other: &Dayjs) -> i64 {
        self.diff_days(other) / 7
    }

    fn diff_months(&self, other: &Dayjs) -> i64 {
        let years = self.time.year() - other.time.year();
        let months = self.time.month() as i32 - other.time.month() as i32;
        (years * 12 + months) as i64
    }

    fn diff_years(&self, other: &Dayjs) -> i64 {
        (self.time.year() - other.time.year()) as i64
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
    /// Add a duration to the current time.
    fn add(&mut self, timestamp: i32) {
        let dt = self.time + chrono::Duration::seconds(timestamp as i64);
        self.time = dt;
    }

    /// Add a duration to the current time in year.
    fn add_years(&mut self, years: i32) {
        let dt = self.time + chrono::Duration::days((years * 365) as i64);
        self.time = dt;
    }

    /// Add a duration to the current time in month.
    fn add_months(&mut self, months: i32) {
        let mut dt = self.time;
        for _ in 0..months {
            dt = dt.with_month(dt.month() + 1).unwrap_or(dt);
        }
        self.time = dt;
    }

    /// Add a duration to the current time in week.
    fn add_weeks(&mut self, weeks: i32) {
        let dt = self.time + chrono::Duration::weeks(weeks as i64);
        self.time = dt;
    }

    /// Add a duration to the current time in day.
    fn add_days(&mut self, days: i32) {
        let dt = self.time + chrono::Duration::days(days as i64);
        self.time = dt;
    }

    /// Add a duration to the current time in hours.
    fn add_hours(&mut self, hours: i32) {
        let dt = self.time + chrono::Duration::hours(hours as i64);
        self.time = dt;
    }

    /// Add a duration to the current time in minutes.
    fn add_minutes(&mut self, minutes: i32) {
        let dt = self.time + chrono::Duration::minutes(minutes as i64);
        self.time = dt;
    }

    /// Add a duration to the current time in seconds.
    fn add_seconds(&mut self, seconds: i32) {
        let dt = self.time + chrono::Duration::seconds(seconds as i64);
        self.time = dt;
    }

    /// Add a duration to the current time in milliseconds.
    fn add_milliseconds(&mut self, milliseconds: i32) {
        let dt = self.time + chrono::Duration::milliseconds(milliseconds as i64);
        self.time = dt;
    }

    /// Subtract a duration from the current time.
    fn subtract(&mut self, timestamp: i32) {
        let dt = self.time - chrono::Duration::seconds(timestamp as i64);
        self.time = dt;
    }

    /// Subtract a duration from the current time in year.
    fn subtract_years(&mut self, years: i32) {
        let dt = self.time - chrono::Duration::days(years as i64);
        self.time = dt;
    }

    /// Subtract a duration from the current time in month.
    fn subtract_months(&mut self, months: i32) {
        let mut dt = self.time;
        for _ in 0..months {
            dt = dt.with_month(dt.month() - 1).unwrap_or(dt);
        }
        self.time = dt;
    }

    /// Subtract a duration from the current time in week.
    fn subtract_weeks(&mut self, weeks: i32) {
        let dt = self.time - chrono::Duration::weeks(weeks as i64);
        self.time = dt;
    }

    /// Subtract a duration from the current time in day.
    fn subtract_days(&mut self, days: i32) {
        let dt = self.time - chrono::Duration::days(days as i64);
        self.time = dt;
    }

    /// Subtract a duration from the current time in hours.
    fn subtract_hours(&mut self, hours: i32) {
        let dt = self.time - chrono::Duration::hours(hours as i64);
        self.time = dt;
    }

    /// Subtract a duration from the current time in minutes.
    fn subtract_minutes(&mut self, minutes: i32) {
        let dt = self.time - chrono::Duration::minutes(minutes as i64);
        self.time = dt;
    }

    /// Subtract a duration from the current time in seconds.
    fn subtract_seconds(&mut self, seconds: i32) {
        let dt = self.time - chrono::Duration::seconds(seconds as i64);
        self.time = dt;
    }

    /// Subtract a duration from the current time in milliseconds.
    fn subtract_milliseconds(&mut self, milliseconds: i32) {
        let dt = self.time - chrono::Duration::milliseconds(milliseconds as i64);
        self.time = dt;
    }
}

/// Create a Dayjs instance from year, month, day, hour, minute, second
pub fn from_ymd(year: i32, month: u32, day: u32) -> Result<Dayjs, String> {
    let date = NaiveDate::from_ymd_opt(year, month, day)
        .ok_or_else(|| format!("Invalid date: {}-{}-{}", year, month, day))?;
    let datetime = date.and_hms_opt(0, 0, 0).ok_or("Invalid time")?;
    Ok(Dayjs {
        time: datetime.and_utc(),
        ..Default::default()
    })
}

/// Create a Dayjs instance from year, month, day, hour, minute, second
pub fn from_ymdhms(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
) -> Result<Dayjs, String> {
    let date = NaiveDate::from_ymd_opt(year, month, day)
        .ok_or_else(|| format!("Invalid date: {}-{}-{}", year, month, day))?;
    let datetime = date
        .and_hms_opt(hour, minute, second)
        .ok_or("Invalid time")?;
    Ok(Dayjs {
        time: datetime.and_utc(),
        ..Default::default()
    })
}

/// Create a Dayjs instance from a JavaScript-style array [year, month, day, hour, minute, second, millisecond]
/// Note: month is 0-indexed (0-11) like JavaScript
pub fn from_array(arr: &[i32]) -> Result<Dayjs, String> {
    if arr.is_empty() {
        return Err("Array is empty".to_string());
    }

    let year = arr[0];
    let month = arr.get(1).copied().unwrap_or(0) + 1; // Convert from 0-indexed
    let day = arr.get(2).copied().unwrap_or(1);
    let hour = arr.get(3).copied().unwrap_or(0);
    let minute = arr.get(4).copied().unwrap_or(0);
    let second = arr.get(5).copied().unwrap_or(0);
    let ms = arr.get(6).copied().unwrap_or(0);

    let date = NaiveDate::from_ymd_opt(year, month as u32, day as u32)
        .ok_or_else(|| format!("Invalid date: {}-{}-{}", year, month, day))?;
    let time = NaiveTime::from_hms_milli_opt(hour as u32, minute as u32, second as u32, ms as u32)
        .ok_or("Invalid time")?;
    let datetime = date.and_time(time);

    Ok(Dayjs {
        time: datetime.and_utc(),
        ..Default::default()
    })
}

/// Create a Dayjs from a chrono `DateTime<Utc>`
pub fn from_datetime(dt: DateTime<Utc>) -> Dayjs {
    Dayjs {
        time: dt,
        ..Default::default()
    }
}

/// Create a Dayjs from a chrono NaiveDateTime (assumes UTC)
pub fn from_naive(ndt: NaiveDateTime) -> Dayjs {
    Dayjs {
        time: ndt.and_utc(),
        ..Default::default()
    }
}

/// Parse with a custom format string
pub fn from_format(s: &str, format: &str) -> Result<Dayjs, String> {
    let ndt = NaiveDateTime::parse_from_str(s, format)
        .map_err(|e| format!("Failed to parse '{}' with format '{}': {}", s, format, e))?;
    Ok(Dayjs {
        time: ndt.and_utc(),
        ..Default::default()
    })
}

/// Get the minimum of two Dayjs instances
pub fn min(a: &Dayjs, b: &Dayjs) -> Dayjs {
    if a.time < b.time {
        a.clone()
    } else {
        b.clone()
    }
}

/// Get the maximum of two Dayjs instances
pub fn max(a: &Dayjs, b: &Dayjs) -> Dayjs {
    if a.time > b.time {
        a.clone()
    } else {
        b.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_from_object_and_to_object() {
        // Test round-trip conversion
        let d1 = dayjs();
        let obj = d1.to_object();
        let d2 = Dayjs::from_object(obj).unwrap();

        assert_eq!(d1.time, d2.time);
        assert_eq!(d1.tz, d2.tz);
    }

    #[test]
    fn test_from_object_with_offset_timezone() {
        let obj = json!({
            "tz": "+08:00",
            "time": "2025-12-21T14:30:00Z"
        });

        let d = Dayjs::from_object(obj).unwrap();
        assert_eq!(d.to_string(), "2025-12-21T14:30:00.000000Z");

        let local = d.to_local();
        assert!(local.contains("22:30:00")); // UTC+8
        assert!(local.contains("+08:00"));
    }

    #[test]
    fn test_from_object_with_number_timezone() {
        let obj = json!({
            "tz": "9",
            "time": "2025-12-21T14:30:00Z"
        });

        let d = Dayjs::from_object(obj).unwrap();
        let local = d.to_local();
        assert!(local.contains("23:30:00")); // UTC+9
        assert!(local.contains("+09:00"));
    }

    #[test]
    fn test_from_object_missing_field() {
        let obj = json!({
            "tz": "+08:00"
        });

        let result = Dayjs::from_object(obj);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("time"));
    }

    #[test]
    fn test_from_object_invalid_time() {
        let obj = json!({
            "tz": "+08:00",
            "time": "invalid-time-format"
        });

        let result = Dayjs::from_object(obj);
        assert!(result.is_err());
    }

    #[test]
    fn test_timezone_from_string() {
        // Test offset format
        let tz1 = TimeZone::from_string("+08:00".to_string());
        assert_eq!(tz1, TimeZone::TimeZoneTime("+08:00".to_string()));

        // Test number format
        let tz2 = TimeZone::from_string("9".to_string());
        assert_eq!(tz2, TimeZone::TimeZoneNumber(9));

        // Test city format
        let tz3 = TimeZone::from_string("Asia/Shanghai".to_string());
        assert_eq!(tz3, TimeZone::TimeZoneCity("Asia/Shanghai".to_string()));
    }
}
