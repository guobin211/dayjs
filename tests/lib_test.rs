#[cfg(test)]
mod tests {
    use chrono::{Datelike, Local, Utc};

    #[test]
    fn test_now() {
        let now = dayjs::now();
        let u_now = Utc::now();
        let l_now = Local::now();
        println!("{now}");
        println!("{u_now}");
        println!("{l_now}");
        assert_eq!(now.date(), u_now.day());
    }
}
