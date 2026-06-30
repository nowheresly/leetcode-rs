
pub struct Solution {}

impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let mut count = vec![0; 3];
        let n = s.len();
        let mut left = 0;
        let mut res:i32 = 0;
        let ch:Vec<char> = s.chars().collect();
        for right in 0..n {
            count[(ch[right] as u8 - b'a') as usize] += 1;
            while count[0] > 0 && count[1] > 0 && count[2] > 0 {
                res += n as i32 - right as i32;
                count[(ch[left] as u8 - b'a') as usize] -= 1;
                left += 1;
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
        assert_eq!(10, Solution::number_of_substrings("abcabc".to_string()));
    }
}
