import dayjs from "dayjs";
import fs from "fs";

// 测试用例集合
const testCases = {
  // ============ Parse Tests ============
  parse: {
    now: {
      description: "Current time",
      js: () => {
        const now = dayjs();
        return {
          isValid: now.isValid(),
          hasTime: now.unix() > 0,
        };
      },
    },

    string_iso: {
      description: "Parse ISO 8601 string",
      js: () => {
        const d = dayjs("2023-05-15T10:30:45Z");
        return {
          year: d.year(),
          month: d.month(),
          date: d.date(),
          hour: d.hour(),
          minute: d.minute(),
          second: d.second(),
          iso: d.toISOString(),
        };
      },
    },

    string_date_only: {
      description: "Parse date only string",
      js: () => {
        const d = dayjs("2023-05-15");
        return {
          year: d.year(),
          month: d.month(),
          date: d.date(),
        };
      },
    },

    unix_milliseconds: {
      description: "Parse Unix timestamp (milliseconds)",
      js: () => {
        const d = dayjs(1684147845000);
        return {
          year: d.year(),
          month: d.month(),
          date: d.date(),
          unix: d.unix(),
          valueOf: d.valueOf(),
        };
      },
    },

    unix_seconds: {
      description: "Parse Unix timestamp (seconds)",
      js: () => {
        const d = dayjs.unix(1684147845);
        return {
          year: d.year(),
          month: d.month(),
          date: d.date(),
          unix: d.unix(),
        };
      },
    },
  },

  // ============ Get + Set Tests ============
  getSet: {
    year: {
      description: "Get/Set year",
      js: () => {
        const d = dayjs("2023-05-15");
        return {
          year: d.year(),
          month: d.month(),
          date: d.date(),
        };
      },
    },

    month: {
      description: "Get month (0-11)",
      js: () => {
        const d = dayjs("2023-05-15");
        return {
          month: d.month(), // 0-11
          date: d.date(),
        };
      },
    },

    date: {
      description: "Get day of month (1-31)",
      js: () => {
        const d = dayjs("2023-05-15");
        return {
          date: d.date(),
        };
      },
    },

    day: {
      description: "Get day of week (0-6)",
      js: () => {
        const d = dayjs("2023-05-15");
        return {
          day: d.day(), // 0=Sunday, 6=Saturday
        };
      },
    },

    hour_minute_second: {
      description: "Get hour, minute, second",
      js: () => {
        const d = dayjs("2023-05-15T10:30:45");
        return {
          hour: d.hour(),
          minute: d.minute(),
          second: d.second(),
          millisecond: d.millisecond(),
        };
      },
    },
  },

  // ============ Manipulate Tests ============
  manipulate: {
    add_days: {
      description: "Add days",
      js: () => {
        const d = dayjs("2023-05-15").add(7, "day");
        return {
          year: d.year(),
          month: d.month(),
          date: d.date(),
          iso: d.toISOString(),
        };
      },
    },

    add_months: {
      description: "Add months",
      js: () => {
        const d = dayjs("2023-05-15").add(2, "month");
        return {
          year: d.year(),
          month: d.month(),
          date: d.date(),
        };
      },
    },

    add_years: {
      description: "Add years",
      js: () => {
        const d = dayjs("2023-05-15").add(1, "year");
        return {
          year: d.year(),
          month: d.month(),
          date: d.date(),
        };
      },
    },

    subtract_days: {
      description: "Subtract days",
      js: () => {
        const d = dayjs("2023-05-15").subtract(7, "day");
        return {
          year: d.year(),
          month: d.month(),
          date: d.date(),
        };
      },
    },

    start_of_day: {
      description: "Start of day",
      js: () => {
        const d = dayjs("2023-05-15T10:30:45").startOf("day");
        return {
          hour: d.hour(),
          minute: d.minute(),
          second: d.second(),
          millisecond: d.millisecond(),
          iso: d.toISOString(),
        };
      },
    },

    end_of_day: {
      description: "End of day",
      js: () => {
        const d = dayjs("2023-05-15T10:30:45").endOf("day");
        return {
          hour: d.hour(),
          minute: d.minute(),
          second: d.second(),
          millisecond: d.millisecond(),
          iso: d.toISOString(),
        };
      },
    },

    start_of_month: {
      description: "Start of month",
      js: () => {
        const d = dayjs("2023-05-15").startOf("month");
        return {
          date: d.date(),
          hour: d.hour(),
          minute: d.minute(),
        };
      },
    },

    end_of_month: {
      description: "End of month",
      js: () => {
        const d = dayjs("2023-05-31").endOf("month");
        return {
          date: d.date(),
          hour: d.hour(),
          minute: d.minute(),
        };
      },
    },

    start_of_year: {
      description: "Start of year",
      js: () => {
        const d = dayjs("2023-05-15").startOf("year");
        return {
          month: d.month(),
          date: d.date(),
        };
      },
    },

    end_of_year: {
      description: "End of year",
      js: () => {
        const d = dayjs("2023-05-15").endOf("year");
        return {
          month: d.month(),
          date: d.date(),
        };
      },
    },
  },

  // ============ Display Tests ============
  display: {
    format_default: {
      description: "Format default",
      js: () => {
        const d = dayjs("2023-05-15T10:30:45");
        return {
          iso: d.toISOString(),
          string: d.toString(),
        };
      },
    },

    format_custom: {
      description: "Format custom",
      js: () => {
        const d = dayjs("2023-05-15T10:30:45");
        return {
          "YYYY-MM-DD": d.format("YYYY-MM-DD"),
          "YYYY/MM/DD": d.format("YYYY/MM/DD"),
          "YYYY-MM-DD HH:mm:ss": d.format("YYYY-MM-DD HH:mm:ss"),
          "DD/MM/YYYY": d.format("DD/MM/YYYY"),
        };
      },
    },

    to_iso: {
      description: "To ISO string",
      js: () => {
        const d = dayjs("2023-05-15T10:30:45.123Z");
        return {
          iso: d.toISOString(),
        };
      },
    },

    to_json: {
      description: "To JSON",
      js: () => {
        const d = dayjs("2023-05-15T10:30:45Z");
        return {
          json: d.toJSON(),
        };
      },
    },

    unix_timestamp: {
      description: "Unix timestamp",
      js: () => {
        const d = dayjs("2023-05-15T10:30:45Z");
        return {
          unix: d.unix(),
          valueOf: d.valueOf(),
        };
      },
    },
  },

  // ============ Query Tests ============
  query: {
    is_before: {
      description: "Is before",
      js: () => {
        const d1 = dayjs("2023-05-15");
        const d2 = dayjs("2023-05-20");
        return {
          result: d1.isBefore(d2),
          reverse: d2.isBefore(d1),
        };
      },
    },

    is_after: {
      description: "Is after",
      js: () => {
        const d1 = dayjs("2023-05-15");
        const d2 = dayjs("2023-05-20");
        return {
          result: d1.isAfter(d2),
          reverse: d2.isAfter(d1),
        };
      },
    },

    is_same: {
      description: "Is same",
      js: () => {
        const d1 = dayjs("2023-05-15");
        const d2 = dayjs("2023-05-15");
        const d3 = dayjs("2023-05-16");
        return {
          same: d1.isSame(d2),
          different: d1.isSame(d3),
        };
      },
    },

    is_same_or_before: {
      description: "Is same or before",
      js: () => {
        const d1 = dayjs("2023-05-15");
        const d2 = dayjs("2023-05-15");
        const d3 = dayjs("2023-05-20");
        return {
          same: d1.isSameOrBefore(d2),
          before: d1.isSameOrBefore(d3),
        };
      },
    },

    is_same_or_after: {
      description: "Is same or after",
      js: () => {
        const d1 = dayjs("2023-05-15");
        const d2 = dayjs("2023-05-15");
        const d3 = dayjs("2023-05-10");
        return {
          same: d1.isSameOrAfter(d2),
          after: d1.isSameOrAfter(d3),
        };
      },
    },

    is_between: {
      description: "Is between",
      js: () => {
        const d1 = dayjs("2023-05-10");
        const d2 = dayjs("2023-05-15");
        const d3 = dayjs("2023-05-20");
        return {
          between: d2.isBetween(d1, d3),
          notBetween: d1.isBetween(d2, d3),
        };
      },
    },
  },

  // ============ Diff Tests ============
  diff: {
    diff_days: {
      description: "Diff in days",
      js: () => {
        const d1 = dayjs("2023-05-15");
        const d2 = dayjs("2023-05-20");
        return {
          diff: d2.diff(d1, "day"),
          reverseDiff: d1.diff(d2, "day"),
        };
      },
    },

    diff_months: {
      description: "Diff in months",
      js: () => {
        const d1 = dayjs("2023-01-15");
        const d2 = dayjs("2023-05-20");
        return {
          diff: d2.diff(d1, "month"),
        };
      },
    },

    diff_years: {
      description: "Diff in years",
      js: () => {
        const d1 = dayjs("2020-05-15");
        const d2 = dayjs("2023-05-15");
        return {
          diff: d2.diff(d1, "year"),
        };
      },
    },

    diff_hours: {
      description: "Diff in hours",
      js: () => {
        const d1 = dayjs("2023-05-15T10:00:00");
        const d2 = dayjs("2023-05-15T15:30:00");
        return {
          diff: d2.diff(d1, "hour"),
        };
      },
    },

    diff_minutes: {
      description: "Diff in minutes",
      js: () => {
        const d1 = dayjs("2023-05-15T10:00:00");
        const d2 = dayjs("2023-05-15T10:30:00");
        return {
          diff: d2.diff(d1, "minute"),
        };
      },
    },

    diff_seconds: {
      description: "Diff in seconds",
      js: () => {
        const d1 = dayjs("2023-05-15T10:00:00");
        const d2 = dayjs("2023-05-15T10:00:45");
        return {
          diff: d2.diff(d1, "second"),
        };
      },
    },
  },

  // ============ Utilities Tests ============
  utilities: {
    days_in_month: {
      description: "Days in month",
      js: () => {
        return {
          january: dayjs("2023-01-15").daysInMonth(),
          february: dayjs("2023-02-15").daysInMonth(),
          february_leap: dayjs("2024-02-15").daysInMonth(),
          march: dayjs("2023-03-15").daysInMonth(),
          april: dayjs("2023-04-15").daysInMonth(),
        };
      },
    },

    is_leap_year: {
      description: "Is leap year",
      js: () => {
        return {
          "2023": dayjs("2023-01-01").isLeapYear(),
          "2024": dayjs("2024-01-01").isLeapYear(),
          "2000": dayjs("2000-01-01").isLeapYear(),
          "1900": dayjs("1900-01-01").isLeapYear(),
        };
      },
    },

    clone: {
      description: "Clone dayjs object",
      js: () => {
        const d1 = dayjs("2023-05-15");
        const d2 = d1.clone();
        return {
          same: d1.isSame(d2),
          year: d2.year(),
        };
      },
    },
  },
};

// 运行所有测试并生成 JSON 输出
function runAllTests() {
  const results = {};

  for (const [category, tests] of Object.entries(testCases)) {
    results[category] = {};

    for (const [testName, test] of Object.entries(tests)) {
      try {
        results[category][testName] = {
          description: test.description,
          result: test.js(),
        };
      } catch (error) {
        results[category][testName] = {
          description: test.description,
          error: error.message,
        };
      }
    }
  }

  return results;
}

// 执行测试
const results = runAllTests();

// 输出结果
console.log(JSON.stringify(results, null, 2));

// 保存到文件
fs.writeFileSync(
  "tests/js_test_results.json",
  JSON.stringify(results, null, 2)
);

console.log("\n✅ JavaScript test results saved to: tests/js_test_results.json");
