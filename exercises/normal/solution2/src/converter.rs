pub fn convert_base(num_str: &str, to_base: u32) -> String {
    let parts: Vec<&str> = num_str.trim_end_matches(')').split('(').collect();
    if parts.len() != 2 {
        return String::new();
    }

    let number = parts[0];
    let from_base = parts[1].parse::<u32>().unwrap_or(10);

    let decimal = match u32::from_str_radix(number, from_base) {
        Ok(num) => num,
        Err(_) => return String::new(),
    };

    if decimal == 0 {
        return "0".to_string();
    }

    let mut result = String::new();
    let mut n = decimal;

    while n > 0 {
        let digit = n % to_base;
        let char = match digit {
            0..=9 => (b'0' + digit as u8) as char,
            10..=35 => (b'a' + (digit - 10) as u8) as char,
            _ => return String::new(),
        };
        result.insert(0, char);
        n /= to_base;
    }

    result
}
