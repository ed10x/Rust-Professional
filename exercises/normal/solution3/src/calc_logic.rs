pub fn new_birthday_probability(n: u32) -> f64 {
    if n == 0 {
        return 0.0;
    }

    let days = 365.0;
    let n = n as f64;

    let mut not_same_prob = 1.0;
    for i in 0..n as u32 {
        not_same_prob *= (days - i as f64) / days;
    }

    ((1.0 - not_same_prob) * 10000.0).round() / 10000.0
}
