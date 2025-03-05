pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    let mut result: u32 = 0;
    let mut num_vec: Vec<u32> = vec![];
    num_vec.push(0);
    num_vec.push(1);
    let mut temp: u32 = 1;
    while temp < threshold {
        num_vec.push(temp);
        temp = num_vec[num_vec.len() - 1] + num_vec[num_vec.len() - 2];
    }

    for i in 0..num_vec.len() {
        temp = num_vec[i];
        if temp % 2 == 1 {
            result += temp;
        }
    }

    return result;
}
