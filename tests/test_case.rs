use dayjs::*;
use serde_json::{Value, json};
use std::fs;

#[test]
fn run_all_tests() {
    let results = generate_test_results();

    // 保存结果到文件
    let json_string = serde_json::to_string_pretty(&results).unwrap();
    fs::write("tests/rust_test_results.json", &json_string).unwrap();

    println!("✅ Rust test results saved to: tests/rust_test_results.json");

    // 读取 JS 测试结果并对比
    if let Ok(js_results_str) = fs::read_to_string("tests/js_test_results.json") {
        println!("\n📊 Comparing with JavaScript results...");
        let js_results: Value = serde_json::from_str(&js_results_str).unwrap();
        compare_and_generate_diff(&results, &js_results);
    }
}

fn generate_test_results() -> Value {
    json!({
        "parse": {
            "now": {
                "description": "Current time",
                "result": test_parse_now(),
            },
            "string_iso": {
                "description": "Parse ISO 8601 string",
                "result": test_parse_string_iso(),
            },
            "string_date_only": {
                "description": "Parse date only string",
                "result": test_parse_string_date_only(),
            },
            "unix_milliseconds": {
                "description": "Parse Unix timestamp (milliseconds)",
                "result": test_parse_unix_milliseconds(),
            },
            "unix_seconds": {
                "description": "Parse Unix timestamp (seconds)",
                "result": test_parse_unix_seconds(),
            },
        },
        "getSet": {
            "year": {
                "description": "Get/Set year",
                "result": test_get_year(),
            },
            "month": {
                "description": "Get month (0-11)",
                "result": test_get_month(),
            },
            "date": {
                "description": "Get day of month (1-31)",
                "result": test_get_date(),
            },
            "day": {
                "description": "Get day of week (0-6)",
                "result": test_get_day(),
            },
            "hour_minute_second": {
                "description": "Get hour, minute, second",
                "result": test_get_hour_minute_second(),
            },
        },
        "manipulate": {
            "add_days": {
                "description": "Add days",
                "result": test_add_days(),
            },
            "add_months": {
                "description": "Add months",
                "result": test_add_months(),
            },
            "add_years": {
                "description": "Add years",
                "result": test_add_years(),
            },
            "subtract_days": {
                "description": "Subtract days",
                "result": test_subtract_days(),
            },
            "start_of_day": {
                "description": "Start of day",
                "result": test_start_of_day(),
            },
            "end_of_day": {
                "description": "End of day",
                "result": test_end_of_day(),
            },
            "start_of_month": {
                "description": "Start of month",
                "result": test_start_of_month(),
            },
            "end_of_month": {
                "description": "End of month",
                "result": test_end_of_month(),
            },
            "start_of_year": {
                "description": "Start of year",
                "result": test_start_of_year(),
            },
            "end_of_year": {
                "description": "End of year",
                "result": test_end_of_year(),
            },
        },
        "display": {
            "format_default": {
                "description": "Format default",
                "result": test_format_default(),
            },
            "format_custom": {
                "description": "Format custom",
                "result": test_format_custom(),
            },
            "to_iso": {
                "description": "To ISO string",
                "result": test_to_iso(),
            },
            "to_json": {
                "description": "To JSON",
                "result": test_to_json(),
            },
            "unix_timestamp": {
                "description": "Unix timestamp",
                "result": test_unix_timestamp(),
            },
        },
        "query": {
            "is_before": {
                "description": "Is before",
                "result": test_is_before(),
            },
            "is_after": {
                "description": "Is after",
                "result": test_is_after(),
            },
            "is_same": {
                "description": "Is same",
                "result": test_is_same(),
            },
            "is_same_or_before": {
                "description": "Is same or before",
                "result": test_is_same_or_before(),
            },
            "is_same_or_after": {
                "description": "Is same or after",
                "result": test_is_same_or_after(),
            },
            "is_between": {
                "description": "Is between",
                "result": test_is_between(),
            },
        },
        "diff": {
            "diff_days": {
                "description": "Diff in days",
                "result": test_diff_days(),
            },
            "diff_months": {
                "description": "Diff in months",
                "result": test_diff_months(),
            },
            "diff_years": {
                "description": "Diff in years",
                "result": test_diff_years(),
            },
            "diff_hours": {
                "description": "Diff in hours",
                "result": test_diff_hours(),
            },
            "diff_minutes": {
                "description": "Diff in minutes",
                "result": test_diff_minutes(),
            },
            "diff_seconds": {
                "description": "Diff in seconds",
                "result": test_diff_seconds(),
            },
        },
        "utilities": {
            "days_in_month": {
                "description": "Days in month",
                "result": test_days_in_month(),
            },
            "is_leap_year": {
                "description": "Is leap year",
                "result": test_is_leap_year(),
            },
            "clone": {
                "description": "Clone dayjs object",
                "result": test_clone(),
            },
        },
    })
}

// ============ Parse Tests ============
fn test_parse_now() -> Value {
    let now = dayjs();
    json!({
        "isValid": now.is_valid(),
        "hasTime": now.unix() > 0,
    })
}

fn test_parse_string_iso() -> Value {
    let d = from_str("2023-05-15T10:30:45Z").unwrap();
    json!({
        "year": d.year(),
        "month": d.month(),
        "date": d.date(),
        "hour": d.hour(),
        "minute": d.minute(),
        "second": d.second(),
        "iso": d.to_iso(),
    })
}

fn test_parse_string_date_only() -> Value {
    let d = from_str("2023-05-15").unwrap();
    json!({
        "year": d.year(),
        "month": d.month(),
        "date": d.date(),
    })
}

fn test_parse_unix_milliseconds() -> Value {
    let d = from_int64(1684147845000).unwrap();
    json!({
        "year": d.year(),
        "month": d.month(),
        "date": d.date(),
        "unix": d.unix(),
        "valueOf": d.value_of(),
    })
}

fn test_parse_unix_seconds() -> Value {
    let d = from_int64(1684147845).unwrap();
    json!({
        "year": d.year(),
        "month": d.month(),
        "date": d.date(),
        "unix": d.unix(),
    })
}

// ============ Get + Set Tests ============
fn test_get_year() -> Value {
    let d = from_str("2023-05-15").unwrap();
    json!({
        "year": d.year(),
        "month": d.month(),
        "date": d.date(),
    })
}

fn test_get_month() -> Value {
    let d = from_str("2023-05-15").unwrap();
    json!({
        "month": d.month(),
        "date": d.date(),
    })
}

fn test_get_date() -> Value {
    let d = from_str("2023-05-15").unwrap();
    json!({
        "date": d.date(),
    })
}

fn test_get_day() -> Value {
    let d = from_str("2023-05-15").unwrap();
    json!({
        "day": d.day().num_days_from_sunday(),
    })
}

fn test_get_hour_minute_second() -> Value {
    let d = from_str("2023-05-15T10:30:45").unwrap();
    json!({
        "hour": d.hour(),
        "minute": d.minute(),
        "second": d.second(),
        "millisecond": d.millisecond(),
    })
}

// ============ Manipulate Tests ============
fn test_add_days() -> Value {
    let d = from_str("2023-05-15").unwrap();
    let mut result = d.clone();
    result.add_days(7);
    json!({
        "year": result.year(),
        "month": result.month(),
        "date": result.date(),
        "iso": result.to_iso(),
    })
}

fn test_add_months() -> Value {
    let d = from_str("2023-05-15").unwrap();
    let mut result = d.clone();
    result.add_months(2);
    json!({
        "year": result.year(),
        "month": result.month(),
        "date": result.date(),
    })
}

fn test_add_years() -> Value {
    let d = from_str("2023-05-15").unwrap();
    let mut result = d.clone();
    result.add_years(1);
    json!({
        "year": result.year(),
        "month": result.month(),
        "date": result.date(),
    })
}

fn test_subtract_days() -> Value {
    let d = from_str("2023-05-15").unwrap();
    let mut result = d.clone();
    result.subtract_days(7);
    json!({
        "year": result.year(),
        "month": result.month(),
        "date": result.date(),
    })
}

fn test_start_of_day() -> Value {
    let d = from_str("2023-05-15T10:30:45").unwrap();
    let result = d.start_of("day");
    json!({
        "hour": result.hour(),
        "minute": result.minute(),
        "second": result.second(),
        "millisecond": result.millisecond(),
        "iso": result.to_iso(),
    })
}

fn test_end_of_day() -> Value {
    let d = from_str("2023-05-15T10:30:45").unwrap();
    let result = d.end_of("day");
    json!({
        "hour": result.hour(),
        "minute": result.minute(),
        "second": result.second(),
        "millisecond": result.millisecond(),
        "iso": result.to_iso(),
    })
}

fn test_start_of_month() -> Value {
    let d = from_str("2023-05-15").unwrap();
    let result = d.start_of("month");
    json!({
        "date": result.date(),
        "hour": result.hour(),
        "minute": result.minute(),
    })
}

fn test_end_of_month() -> Value {
    let d = from_str("2023-05-31").unwrap();
    let result = d.end_of("month");
    json!({
        "date": result.date(),
        "hour": result.hour(),
        "minute": result.minute(),
    })
}

fn test_start_of_year() -> Value {
    let d = from_str("2023-05-15").unwrap();
    let result = d.start_of("year");
    json!({
        "month": result.month(),
        "date": result.date(),
    })
}

fn test_end_of_year() -> Value {
    let d = from_str("2023-05-15").unwrap();
    let result = d.end_of("year");
    json!({
        "month": result.month(),
        "date": result.date(),
    })
}

// ============ Display Tests ============
fn test_format_default() -> Value {
    let d = from_str("2023-05-15T10:30:45").unwrap();
    json!({
        "iso": d.to_iso(),
        "string": d.to_string(),
    })
}

fn test_format_custom() -> Value {
    let d = from_str("2023-05-15T10:30:45").unwrap();
    json!({
        "YYYY-MM-DD": d.format("%Y-%m-%d"),
        "YYYY/MM/DD": d.format("%Y/%m/%d"),
        "YYYY-MM-DD HH:mm:ss": d.format("%Y-%m-%d %H:%M:%S"),
        "DD/MM/YYYY": d.format("%d/%m/%Y"),
    })
}

fn test_to_iso() -> Value {
    let d = from_str("2023-05-15T10:30:45.123Z").unwrap();
    json!({
        "iso": d.to_iso(),
    })
}

fn test_to_json() -> Value {
    let d = from_str("2023-05-15T10:30:45Z").unwrap();
    json!({
        "json": d.to_iso(),
    })
}

fn test_unix_timestamp() -> Value {
    let d = from_str("2023-05-15T10:30:45Z").unwrap();
    json!({
        "unix": d.unix(),
        "valueOf": d.value_of(),
    })
}

// ============ Query Tests ============
fn test_is_before() -> Value {
    let d1 = from_str("2023-05-15").unwrap();
    let d2 = from_str("2023-05-20").unwrap();
    json!({
        "result": d1.is_before(&d2),
        "reverse": d2.is_before(&d1),
    })
}

fn test_is_after() -> Value {
    let d1 = from_str("2023-05-15").unwrap();
    let d2 = from_str("2023-05-20").unwrap();
    json!({
        "result": d1.is_after(&d2),
        "reverse": d2.is_after(&d1),
    })
}

fn test_is_same() -> Value {
    let d1 = from_str("2023-05-15").unwrap();
    let d2 = from_str("2023-05-15").unwrap();
    let d3 = from_str("2023-05-16").unwrap();
    json!({
        "same": d1.is_same(&d2),
        "different": d1.is_same(&d3),
    })
}

fn test_is_same_or_before() -> Value {
    let d1 = from_str("2023-05-15").unwrap();
    let d2 = from_str("2023-05-15").unwrap();
    let d3 = from_str("2023-05-20").unwrap();
    json!({
        "same": d1.is_same_or_before(&d2),
        "before": d1.is_same_or_before(&d3),
    })
}

fn test_is_same_or_after() -> Value {
    let d1 = from_str("2023-05-15").unwrap();
    let d2 = from_str("2023-05-15").unwrap();
    let d3 = from_str("2023-05-10").unwrap();
    json!({
        "same": d1.is_same_or_after(&d2),
        "after": d1.is_same_or_after(&d3),
    })
}

fn test_is_between() -> Value {
    let d1 = from_str("2023-05-10").unwrap();
    let d2 = from_str("2023-05-15").unwrap();
    let d3 = from_str("2023-05-20").unwrap();
    json!({
        "between": d2.is_between(&d1, &d3),
        "notBetween": d1.is_between(&d2, &d3),
    })
}

// ============ Diff Tests ============
fn test_diff_days() -> Value {
    let d1 = from_str("2023-05-15").unwrap();
    let d2 = from_str("2023-05-20").unwrap();
    json!({
        "diff": d2.diff_days(&d1),
        "reverseDiff": d1.diff_days(&d2),
    })
}

fn test_diff_months() -> Value {
    let d1 = from_str("2023-01-15").unwrap();
    let d2 = from_str("2023-05-20").unwrap();
    json!({
        "diff": d2.diff_months(&d1),
    })
}

fn test_diff_years() -> Value {
    let d1 = from_str("2020-05-15").unwrap();
    let d2 = from_str("2023-05-15").unwrap();
    json!({
        "diff": d2.diff_years(&d1),
    })
}

fn test_diff_hours() -> Value {
    let d1 = from_str("2023-05-15T10:00:00").unwrap();
    let d2 = from_str("2023-05-15T15:30:00").unwrap();
    json!({
        "diff": d2.diff_hours(&d1),
    })
}

fn test_diff_minutes() -> Value {
    let d1 = from_str("2023-05-15T10:00:00").unwrap();
    let d2 = from_str("2023-05-15T10:30:00").unwrap();
    json!({
        "diff": d2.diff_minutes(&d1),
    })
}

fn test_diff_seconds() -> Value {
    let d1 = from_str("2023-05-15T10:00:00").unwrap();
    let d2 = from_str("2023-05-15T10:00:45").unwrap();
    json!({
        "diff": d2.diff_seconds(&d1),
    })
}

// ============ Utilities Tests ============
fn test_days_in_month() -> Value {
    json!({
        "january": from_str("2023-01-15").unwrap().days_in_month(),
        "february": from_str("2023-02-15").unwrap().days_in_month(),
        "february_leap": from_str("2024-02-15").unwrap().days_in_month(),
        "march": from_str("2023-03-15").unwrap().days_in_month(),
        "april": from_str("2023-04-15").unwrap().days_in_month(),
    })
}

fn test_is_leap_year() -> Value {
    json!({
        "2023": from_str("2023-01-01").unwrap().is_leap_year(),
        "2024": from_str("2024-01-01").unwrap().is_leap_year(),
        "2000": from_str("2000-01-01").unwrap().is_leap_year(),
        "1900": from_str("1900-01-01").unwrap().is_leap_year(),
    })
}

fn test_clone() -> Value {
    let d1 = from_str("2023-05-15").unwrap();
    let d2 = d1.clone_dayjs();
    json!({
        "same": d1.is_same(&d2),
        "year": d2.year(),
    })
}

// 对比结果并生成 diff 文件
fn compare_and_generate_diff(rust_results: &Value, js_results: &Value) {
    let mut diff_output = String::new();
    diff_output.push_str("=".repeat(80).as_str());
    diff_output.push_str("\n");
    diff_output.push_str("Day.js Rust vs JavaScript API Test Comparison\n");
    diff_output.push_str("=".repeat(80).as_str());
    diff_output.push_str("\n\n");

    let mut passed = 0;
    let mut failed = 0;
    let mut total = 0;

    for (category, tests) in rust_results.as_object().unwrap() {
        diff_output.push_str(&format!("\n[{}]\n", category.to_uppercase()));
        diff_output.push_str("-".repeat(80).as_str());
        diff_output.push_str("\n");

        if let Some(js_category) = js_results.get(category) {
            for (test_name, rust_test) in tests.as_object().unwrap() {
                total += 1;

                diff_output.push_str(&format!("\nTest: {}\n", test_name));
                diff_output.push_str(&format!("Description: {}\n", rust_test["description"]));

                if let Some(js_test) = js_category.get(test_name) {
                    let rust_result = &rust_test["result"];
                    let js_result_value = js_test.get("result");
                    let js_error = js_test.get("error");

                    if let Some(js_result) = js_result_value {
                        if rust_result == js_result {
                            passed += 1;
                            diff_output.push_str("Status: ✅ PASSED\n");
                        } else {
                            failed += 1;
                            diff_output.push_str("Status: ❌ FAILED\n");
                            diff_output.push_str(&format!(
                                "Rust:   {}\n",
                                serde_json::to_string_pretty(rust_result).unwrap()
                            ));
                            diff_output.push_str(&format!(
                                "JS:     {}\n",
                                serde_json::to_string_pretty(js_result).unwrap()
                            ));
                        }
                    } else if let Some(error) = js_error {
                        diff_output.push_str("Status: ⚠️  JS ERROR\n");
                        diff_output.push_str(&format!("JS Error: {}\n", error));
                        diff_output.push_str(&format!(
                            "Rust:     {}\n",
                            serde_json::to_string_pretty(rust_result).unwrap()
                        ));
                    }
                } else {
                    diff_output.push_str("Status: ⚠️  NOT FOUND IN JS\n");
                }
            }
        }
        diff_output.push_str("\n");
    }

    diff_output.push_str("\n");
    diff_output.push_str("=".repeat(80).as_str());
    diff_output.push_str("\n");
    diff_output.push_str("SUMMARY\n");
    diff_output.push_str("=".repeat(80).as_str());
    diff_output.push_str("\n");
    diff_output.push_str(&format!("Total Tests:  {}\n", total));
    diff_output.push_str(&format!(
        "Passed:       {} ({}%)\n",
        passed,
        if total > 0 { (passed * 100) / total } else { 0 }
    ));
    diff_output.push_str(&format!(
        "Failed:       {} ({}%)\n",
        failed,
        if total > 0 { (failed * 100) / total } else { 0 }
    ));
    diff_output.push_str("=".repeat(80).as_str());
    diff_output.push_str("\n");

    // 保存 diff 文件
    fs::write("tests/diff.txt", &diff_output).unwrap();
    println!("✅ Diff report saved to: tests/diff.txt");

    // 在控制台输出简要统计
    let separator = "=".repeat(50);
    println!("\n{}", separator);
    println!("📊 Test Summary:");
    println!("   Total:  {}", total);
    println!(
        "   Passed: {} ({}%)",
        passed,
        if total > 0 { (passed * 100) / total } else { 0 }
    );
    println!(
        "   Failed: {} ({}%)",
        failed,
        if total > 0 { (failed * 100) / total } else { 0 }
    );
    println!("{}", separator);
}
