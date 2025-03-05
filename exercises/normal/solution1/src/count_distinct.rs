use std::collections::HashSet;
pub fn new_count_distinct(input_str: &str) -> usize {
    let words: Vec<&str> = input_str.split(',').collect();
    let mut count: HashSet<&str> = HashSet::new();
    for i in 0..words.len() {
        count.insert(words[i]);
    }
    return count.len();
}
