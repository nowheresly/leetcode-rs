use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut set:HashSet<char> = HashSet::new();
        let mut special_chars:HashSet<char> = HashSet::new();
        let lower:Vec<char> = word.to_lowercase().chars().collect();
        let word:Vec<char> = word.chars().collect();
        for i in 0..word.len() {
            if lower[i] == word[i] {
                if set.contains(&word[i].to_ascii_uppercase()) {
                    special_chars.insert(lower[i]);
                }
            } else {
                if set.contains(&word[i].to_ascii_lowercase()) {
                    special_chars.insert(lower[i]);
                }
            }
            set.insert(word[i]);
        }
        special_chars.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::number_of_special_chars(String::from("aaAbcBC")));

    }
}
