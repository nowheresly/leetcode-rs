
pub struct Solution {}

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut res = 0;
        let mut open = 0;
        for c in s.chars() {
            if c == '(' {
                open += 1;
            } else {
                if open > 0 {
                    open -= 1;
                } else {
                    res += 1;
                }
            }
        }
        res + open
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::min_add_to_make_valid(String::from("()))((")));
    }
}
