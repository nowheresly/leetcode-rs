pub struct Solution {}

impl Solution {
    pub fn beauty_sum(s: String) -> i32 {
        let n = s.len();
        let ch = s.as_bytes();
        let mut res = 0;

        for i in 0..n {
            let mut freq = vec![0;26];
            for j in i..n {
                freq[ch[j] as usize - 'a' as usize] += 1;
                let mut max_freq = 0;
                let mut min_freq = i32::MAX;

                for &count in freq.iter() {
                    if count > 0 {
                        min_freq = min_freq.min(count);
                        max_freq = max_freq.max(count);
                    }
                }

                res += max_freq - min_freq;
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
        let s: String = "aabcb".to_string();
        assert_eq!(5, Solution::beauty_sum(s));
    }
}
