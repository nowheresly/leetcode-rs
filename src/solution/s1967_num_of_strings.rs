
pub struct Solution {}

impl Solution {
    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {

        let mut count = 0;
        for pattern in patterns {
            if word.contains(&pattern) {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {

        assert_eq!(3, Solution::num_of_strings(vec![String::from("a"), String::from("abc"), String::from("bc"), String::from("d")], String::from("abc")));
    }
}
