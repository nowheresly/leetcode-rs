pub struct Solution {}

impl Solution {
    pub fn max_active_sections_after_trade(s: String) -> i32 {
        let ch = s.as_bytes();
        let n = s.len();
        let mut res = 0;
        let mut i = 0;
        let mut block1 = 0;
        let mut block2 = 0;
        let mut max = 0;

        while i < n {
            let c = ch[i] as char;

            if c == '0' {
                block1 += 1;
                i += 1;
                continue;
            }

            while i < n && ch[i] as char == '1' {
                i += 1;
                res += 1;
            }
            while i < n && ch[i] as char == '0' {
                i += 1;
                block2 += 1;
            }
            
            if block1 != 0 && block2 != 0 {
                max = max.max(block1 + block2);
            }
            block1 = block2;
            block2 = 0;
        }
        res + max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(7, Solution::max_active_sections_after_trade(String::from("1000100")));
    }
}
