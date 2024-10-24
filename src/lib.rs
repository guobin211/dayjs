use chrono::{TimeZone, Utc};
use std::fmt::{Display, Formatter};

/// get dayjs instance
///
/// # Examples
///
/// ```
/// let now = dayjs::dayjs();
/// println!("{}", now);
/// ```
pub fn dayjs() -> Dayjs {
    Dayjs::default()
}

/// get dayjs instance from str
pub fn from_str(s: &str) -> Dayjs {
    let time: chrono::DateTime<Utc> = chrono::DateTime::parse_from_rfc3339(s)
        .unwrap()
        .with_timezone(&Utc);
    Dayjs {
        time,
        ..Default::default()
    }
}

/// get dayjs instance from number
pub fn from_timestamp(n: i64) -> Dayjs {
    let len = format!("{}", n).len();
    let time = match len {
        10 => Utc.timestamp_opt(n, 0).unwrap(),
        13 => Utc.timestamp_millis_opt(n).unwrap(),
        _ => Utc::now(),
    };
    Dayjs {
        time,
        ..Default::default()
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DayjsTimeZone {
    DEFAULT,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Dayjs {
    pub tz: DayjsTimeZone,
    time: chrono::DateTime<Utc>,
}

impl Default for Dayjs {
    fn default() -> Self {
        Dayjs {
            tz: DayjsTimeZone::DEFAULT,
            time: Utc::now(),
        }
    }
}

impl Display for Dayjs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.time.to_rfc3339())
    }
}
