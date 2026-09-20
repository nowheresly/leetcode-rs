pub struct Solution {}

impl Solution {
    pub fn reverse_degree(s: String) -> i32 {
        let mut res = 0;
        let mut pos = 1;
        for c in s.chars() {
            let val = (26 - (c as u8 - 'a' as u8) as i32) * pos;
            res += val;
            pos += 1;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(148, Solution::reverse_degree(String::from("abc")));
    }
}
