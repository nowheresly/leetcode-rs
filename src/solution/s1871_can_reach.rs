use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        let n = s.len() as i32;
        let s = s.into_bytes();
        if s[(n - 1) as usize] == b'1' {
            return false;
        }
        let mut queue: VecDeque<i32> = VecDeque::new();
        queue.push_back(0);
        let mut sofar = 0;
        while !queue.is_empty() {
            let i = queue.pop_front().unwrap();

            let start = i32::max(sofar + 1, i + min_jump);
            for j in start..=i32::min(n - 1, i + max_jump) {
                if s[j as usize] == b'0' {
                    queue.push_back(j);
                    if j == n-1 {
                        return true;
                    }
                }
            }
            sofar = i + max_jump;
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(false,  Solution::can_reach(String::from("01101110"), 2, 3));
    }
}
