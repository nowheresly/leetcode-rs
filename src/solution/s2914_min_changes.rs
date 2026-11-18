pub struct Solution {}

impl Solution {
    pub fn min_changes(s: String) -> i32 {
        let mut res = 0;
        let ch = s.chars().collect::<Vec<char>>();
        for i in (0..s.len()).step_by(2) {
            if ch[i] != ch[i + 1] {
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
        let s: String = "1001".to_string();
        assert_eq!(2, Solution::min_changes(s));
    }
}
