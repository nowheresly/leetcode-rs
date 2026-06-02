pub struct Solution {}

impl Solution {
    pub fn count_good_substrings(s: String) -> i32 {
        let mut res = 0;
        let ch:Vec<char> = s.chars().collect();
        for i in 2..s.len() {
            if ch[i-2] != ch[i-1] && ch[i-2] != ch[i] && ch[i-1] != ch[i] {
                res += 1;
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
        assert_eq!(1,  Solution::count_good_substrings(String::from("xyzzaz")));
    }
}
