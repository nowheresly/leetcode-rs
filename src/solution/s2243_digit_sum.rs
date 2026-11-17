use std::fmt::Write;
pub struct Solution {}

impl Solution {
    pub fn digit_sum(mut s: String, k: i32) -> String {
        let k = k as usize;
        while s.len() > k {
            let mut sb = String::new();
            let mut i = 0;
            let mut sum = 0;
            let mut take = false;

            for c in s.chars() {
                sum += (c as u8 - b'0') as u32;
                i += 1;
                take = true;
                if i == k {
                    i = 0;
                    let _ = write!(sb, "{}", sum);
                    sum = 0;
                    take = false;
                }
            }

            if take {
                let _ = write!(sb, "{}", sum);
            }
            s = sb;
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s: String = "11111222223".to_string();
        let k: i32 = 3;
        assert_eq!("135", Solution::digit_sum(s, k));

    }
}
