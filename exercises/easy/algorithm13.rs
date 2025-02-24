/*
    Anagram Check
    Given two strings, check if they are anagrams of each other.
    Anagrams are words or phrases formed by rearranging the letters of another,
    using all the original letters exactly once.
    The strings may contain spaces or punctuation, but you need to ignore them while checking.

    You need to implement the function `are_anagrams(s1: String, s2: String) -> bool`.
    The function should return `true` if the two strings are anagrams, and `false` otherwise.

    Hint: Consider normalizing the strings by removing non-alphabetical characters and converting to lowercase before checking.
*/

use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

pub fn are_anagrams(s1: String, s2: String) -> bool {
    // TODO: Implement the logic to check if two strings are anagrams
    let temp1: String = s1.chars().filter(|c| c.is_alphabetic()).collect();
    let x1 = temp1.to_ascii_uppercase();
    let a1 = x1.chars().collect::<Vec<_>>();
    let mut map1: HashMap<char, i32> = HashMap::new();
    for c in a1 {
        if map1.contains_key(&c) {
            if let Some(value) = map1.get_mut(&c) {
                *value += 1;
            }
        } else {
            map1.insert(c, 1);
        }
    }

    let temp2: String = s2.chars().filter(|c| c.is_alphabetic()).collect();
    let x2 = temp2.to_ascii_uppercase();
    let a2 = x2.chars().collect::<Vec<_>>();
    let mut map2: HashMap<char, i32> = HashMap::new();
    for c in a2 {
        if map2.contains_key(&c) {
            if let Some(value) = map2.get_mut(&c) {
                *value += 1;
            }
        } else {
            map2.insert(c, 1);
        }
    }

    for key in map1.keys() {
        if map2.contains_key(key) {
            if let Some(value2) = map2.get(key) {
                if let Some(value1) = map1.get(key) {
                    if value1 != value2 {
                        return false;
                    }
                }
            }
        } else {
            return false;
        }
    }

    true // Placeholder return value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anagram_1() {
        let s1 = "listen".to_string();
        let s2 = "silent".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_anagram_2() {
        let s1 = "evil".to_string();
        let s2 = "vile".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_anagram_3() {
        let s1 = "hello".to_string();
        let s2 = "world".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, false);
    }

    #[test]
    fn test_anagram_4() {
        let s1 = "Clint Eastwood".to_string();
        let s2 = "Old West Action".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_anagram_5() {
        let s1 = "Astronomer".to_string();
        let s2 = "Moon starer".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }
}
