pub fn dp_rec_mc(amount: u32) -> u32 {
    let mut result: u32 = 0;
    let mut amount_temp = amount;
    while amount_temp > 0 {
        if amount_temp >= 100 {
            amount_temp -= 100;
        } else if amount_temp < 100 && amount_temp >= 50 {
            amount_temp -= 50;
        } else if amount_temp < 50 && amount_temp >= 30 {
            amount_temp -= 30;
        } else if amount_temp < 30 && amount_temp >= 20 {
            amount_temp -= 20;
        } else if amount_temp < 20 && amount_temp >= 10 {
            amount_temp -= 10;
        } else if amount_temp < 10 && amount_temp >= 5 {
            amount_temp -= 5;
        } else if amount_temp < 5 && amount_temp >= 2 {
            amount_temp -= 2;
        } else {
            amount_temp -= 1;
        }
        result += 1;
    }

    return result;
}
