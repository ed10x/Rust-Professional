use std::cmp::min;

fn calc(
    year: i32,
    month: i32,
    old_work_years: i32,
    start_delay_year: i32,
    max_delay_months: i32,
) -> String {
    if year < start_delay_year {
        return format!(
            "{:04}-{:02},{},0",
            year + old_work_years,
            month,
            old_work_years,
        );
    }

    let age_months_from_oracle = (year - start_delay_year) * 12 + month - 1;
    let delay_months = min(age_months_from_oracle / 4 + 1, max_delay_months);
    let retire_months_from_oracle = age_months_from_oracle + old_work_years * 12 + delay_months;

    let (retire_year, retire_month) = (
        start_delay_year + retire_months_from_oracle / 12,
        retire_months_from_oracle % 12 + 1
    );

    let retire_age_months = retire_months_from_oracle - age_months_from_oracle;
    let retire_age_str = if retire_age_months % 12 == 0 {
        (retire_age_months / 12).to_string()
    } else {
        format!("{:.2}", retire_age_months as f64 / 12.0)
    };

    format!(
        "{:04}-{:02},{},{}",
        retire_year,
        retire_month,
        retire_age_str,
        delay_months
    )
}

pub fn retire_time(time: &str, tp: &str) -> String {
    let parts: Vec<&str> = time.split('-').collect();
    let year = parts[0].parse().unwrap_or(0);
    let month = parts[1].parse().unwrap_or(0);

    match tp {
        "男职工" => calc(year, month, 60, 1965, 36),
        "原法定退休年龄55周岁女职工" => calc(year, month, 55, 1970, 36),
        "原法定退休年龄50周岁女职工" => calc(year, month, 50, 1975, 60),
        _ => "未知".to_string(),
    }
}