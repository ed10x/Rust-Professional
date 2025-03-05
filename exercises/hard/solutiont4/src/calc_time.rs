// 判断是否为闰年
fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

// 获取每个月的天数
fn days_in_month(year: i32, month: i32) -> i32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => panic!("Invalid month"),
    }
}

// 计算当年的第几天
fn day_of_year(year: i32, month: i32, day: i32) -> i32 {
    let mut days = 0;
    for m in 1..month {
        days += days_in_month(year, m);
    }
    days + day
}

// 计算当年剩余天数
fn days_left_in_year(year: i32, month: i32, day: i32) -> i32 {
    let total_days = if is_leap_year(year) { 366 } else { 365 };
    total_days - day_of_year(year, month, day)
}

// 计算星期几，使用蔡氏公式
fn day_of_week(year: i32, month: i32, day: i32) -> i32 {
    let mut y = year;
    let mut m = month;
    if m < 3 {
        m += 12;
        y -= 1;
    }
    let c = y / 100;
    y %= 100;
    let w = (y + y / 4 + c / 4 - 2 * c + 26 * (m + 1) / 10 + day - 1) % 7;
    if w < 0 {
        w + 7
    } else if w == 0 {
        7
    } else {
        w
    }
}

// 计算自纪元以来的天数
fn days_since_epoch(year: i32, month: i32, day: i32) -> i32 {
    let mut days = 0;
    for y in 1..year {
        days += if is_leap_year(y) { 366 } else { 365 };
    }
    for m in 1..month {
        days += days_in_month(year, m);
    }
    days + day
}

// 计算两个日期之间的天数差
fn day_difference(year1: i32, month1: i32, day1: i32, year2: i32, month2: i32, day2: i32) -> i32 {
    let days1 = days_since_epoch(year1, month1, day1);
    let days2 = days_since_epoch(year2, month2, day2);
    (days2 - days1).abs()
}

// 计算给定年份的第一个星期一的日期
fn first_monday_of_year(year: i32) -> (i32, i32, i32) {
    let mut first_thursday_day = 1;
    for i in 0..6 {
        let weekday = day_of_week(year, 1, 1 + i);
        if weekday == 4 {
            first_thursday_day = 1 + i;
            break;
        }
    }
    if first_thursday_day <= 3 {
        (year - 1, 12, 31 - (3 - first_thursday_day))
    } else {
        (year, 1, first_thursday_day - 3)
    }
}

// 计算给定日期是该年的第几周
fn week_of_year(year: i32, month: i32, day: i32) -> i32 {
    let (first_monday_year, first_monday_month, first_monday_day) = first_monday_of_year(year);
    let day_diff = day_difference(
        year,
        month,
        day,
        first_monday_year,
        first_monday_month,
        first_monday_day,
    );
    let week_diff = day_diff / 7;

    // 处理特殊情况
    if month == 12 && (29..=31).contains(&day) {
        let (next_monday_year, next_monday_month, next_monday_day) = first_monday_of_year(year + 1);
        if next_monday_month == 12 && next_monday_day <= day {
            return 1;
        }
    }
    if month == 1 && (1..=4).contains(&day) {
        let (first_monday_year, first_monday_month, first_monday_day) = first_monday_of_year(year);
        if first_monday_month == 1 && first_monday_day > day {
            let (last_monday_year, last_monday_month, last_monday_day) =
                first_monday_of_year(year - 1);
            return 1 + day_difference(
                year,
                month,
                day,
                last_monday_year,
                last_monday_month,
                last_monday_day,
            ) / 7;
        }
    }

    1 + week_diff
}

// 计算给定日期距离中国新年的天数
fn days_to_chinese_new_year(year: i32, month: i32, day: i32) -> i32 {
    let current_day = day_of_year(year, month, day);
    let new_year_day = day_of_year(year, 1, 29);
    if current_day < new_year_day {
        new_year_day - current_day
    } else {
        let next_year = year + 1;
        let next_new_year_day = day_of_year(next_year, 2, 17);
        let days_in_current_year = if is_leap_year(year) { 366 } else { 365 };
        days_in_current_year - current_day + next_new_year_day
    }
}

// 计算给定日期距离下一个 A 股开盘日的天数
fn days_to_next_a_share_opening(year: i32, month: i32, day: i32) -> i32 {
    let new_year_day_open = day_of_year(year, 1, 2);
    let spring_year_day_open = day_of_year(year, 2, 5);
    let qingming_day_open = day_of_year(year, 4, 7);
    let labor_day_open = day_of_year(year, 5, 6);
    let zongzi_day_open = day_of_year(year, 6, 3);
    let autumn_day_open = day_of_year(year, 10, 9);
    let next_new_year_day_open = day_of_year(year + 1, 1, 1);

    // 处理节假日
    if month == 1 && day == 1 {
        return new_year_day_open - day_of_year(year, month, day) - 1;
    }
    if (month == 1 && (28..=31).contains(&day)) || (month == 2 && (1..=4).contains(&day)) {
        return spring_year_day_open - day_of_year(year, month, day) - 1;
    }
    if month == 4 && (4..=6).contains(&day) {
        return qingming_day_open - day_of_year(year, month, day) - 1;
    }
    if month == 5 && (1..=5).contains(&day) {
        return labor_day_open - day_of_year(year, month, day) - 1;
    }
    if (month == 5 && day == 31) || (month == 6 && (1..=2).contains(&day)) {
        return zongzi_day_open - day_of_year(year, month, day) - 1;
    }
    if month == 10 && (1..=8).contains(&day) {
        return autumn_day_open - day_of_year(year, month, day) - 1;
    }
    if month == 12 && day == 31 {
        let days = if is_leap_year(year) { 366 } else { 365 };
        return next_new_year_day_open - day_of_year(year, month, day) + days;
    }

    let weekday = day_of_week(year, month, day);
    match weekday {
        7 => 0, // 周日
        5 => 2, // 周五，距离下周一开盘 2 天
        6 => 1, // 周六，距离周一开盘 1 天
        _ => 0, // 周一到周四
    }
}

// 解析日期字符串并进行计算
fn calculate_time(date_str: &str) -> String {
    let parts: Vec<&str> = date_str.split('-').collect();
    if parts.len() != 3 {
        panic!("Invalid date format, expected YYYY-MM-DD");
    }
    let year: i32 = parts[0].parse().expect("Invalid year");
    let month: i32 = parts[1].parse().expect("Invalid month");
    let day: i32 = parts[2].parse().expect("Invalid day");

    let week_num = week_of_year(year, month, day);
    let weekday = day_of_week(year, month, day);
    let day_of_year = day_of_year(year, month, day);
    let days_left = days_left_in_year(year, month, day);
    let days_to_cny = days_to_chinese_new_year(year, month, day);
    let days_to_a_share = days_to_next_a_share_opening(year, month, day);

    format!(
        "{},{},{},{},{},{}",
        week_num, weekday, day_of_year, days_left, days_to_cny, days_to_a_share
    )
}

pub fn time_info(time: &str) -> String {
    calculate_time(time)
}