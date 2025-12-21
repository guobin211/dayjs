use dayjs::{Dayjs, DisplayTime, dayjs};
use serde_json::json;

#[test]
fn test_object() {
    println!("=== Test to_object and from_object ===\n");

    // Test 1: Convert current time to object and back
    let d1 = dayjs();
    println!("Original Dayjs:");
    println!("  to_string: {}", d1.to_string());

    let obj1 = d1.to_object();
    println!("  to_object: {}", obj1);

    let d1_restored = Dayjs::from_object(obj1).unwrap();
    println!("  restored:  {}", d1_restored.to_string());
    println!();

    // Test 2: Create from custom JSON object with offset timezone
    let obj2 = json!({
        "tz": "+08:00",
        "time": "2025-12-21T14:30:00Z"
    });
    println!("Custom object (offset timezone): {}", obj2);
    let d2 = Dayjs::from_object(obj2).unwrap();
    println!("  to_string: {}", d2.to_string());
    println!("  to_local:  {}", d2.to_local());
    println!();

    // Test 3: Create from JSON object with number timezone
    let obj3 = json!({
        "tz": "9",
        "time": "2025-12-21T14:30:00Z"
    });
    println!("Custom object (number timezone): {}", obj3);
    let d3 = Dayjs::from_object(obj3).unwrap();
    println!("  to_string: {}", d3.to_string());
    println!("  to_local:  {}", d3.to_local());
    println!();

    // Test 4: Create from JSON object with city timezone
    let obj4 = json!({
        "tz": "Asia/Shanghai",
        "time": "2025-12-21T14:30:00.123456Z"
    });
    println!("Custom object (city timezone): {}", obj4);
    let d4 = Dayjs::from_object(obj4).unwrap();
    println!("  to_string: {}", d4.to_string());
    println!("  to_local:  {}", d4.to_local());
    println!();

    // Test 5: Error handling - missing field
    let obj5 = json!({
        "tz": "+08:00"
    });
    println!("Invalid object (missing time): {}", obj5);
    match Dayjs::from_object(obj5) {
        Ok(_) => println!("  Unexpected success"),
        Err(e) => println!("  Error: {}", e),
    }
    println!();

    // Test 6: Error handling - invalid time
    let obj6 = json!({
        "tz": "+08:00",
        "time": "invalid-time"
    });
    println!("Invalid object (bad time): {}", obj6);
    match Dayjs::from_object(obj6) {
        Ok(_) => println!("  Unexpected success"),
        Err(e) => println!("  Error: {}", e),
    }
}
