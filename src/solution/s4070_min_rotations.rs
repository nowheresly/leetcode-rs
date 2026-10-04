pub struct Solution {}

impl Solution {
    pub fn min_rotations(s: String) -> i32 {
        let mut res = 0;
        let mut prev = 0;

        for c in s.chars() {
            let cur = (c as u8 - '0' as u8) as i32;

            let diff = i32::abs(cur - prev);

            res += diff.min(10 - diff);

            prev = cur;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(25, Solution::min_rotations(String::from("0192837465")));
    }
}
