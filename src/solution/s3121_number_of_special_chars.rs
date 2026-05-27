pub struct Solution {}

impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut state = [0; 26];
        let mut count = 0;

        for c in word.chars() {
            let is_lower = c.is_ascii_lowercase();
            let idx = (c.to_ascii_lowercase() as usize) - ('a' as usize);

            match (is_lower, state[idx]) {
                (true, 0) => {
                    state[idx] = 1;
                }

                (true, 2) => {
                    state[idx] = -1;
                    count -= 1;
                }

                (false, 1) => {
                    state[idx] = 2;
                    count += 1;
                }

                (false, 0) => {
                    state[idx] = -1;
                }

                _ => {}
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
        assert_eq!(3, Solution::number_of_special_chars(String::from("aaAbcBC")));
        assert_eq!(0, Solution::number_of_special_chars(String::from("AbBCab")));

    }
}
