
pub struct Solution {}
impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut res = 0;
        let mut count = 0;
        for c in s.chars() {
            if c== '(' {
                count += 1;
                res = res.max(count);
            } else if c == ')' {
                count -= 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::max_depth(String::from("()(())((()()))")));
    }
}
