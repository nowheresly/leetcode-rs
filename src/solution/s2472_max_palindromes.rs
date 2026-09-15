
pub struct Solution {}

impl Solution {
    pub fn max_palindromes(s: String, k: i32) -> i32 {
        let n = s.len();
        let k = k as usize;
        let ch = s.as_bytes();
        let mut dp = vec![0; n+1];

        for i in 1..=n {
            dp[i] = dp[i-1];

            if i >= k && is_palin(&ch, i - k, i - 1) {
                dp[i] = dp[i].max(dp[i-k] + 1);
            }

            if i >= k + 1 && is_palin(ch, i - (k + 1), i - 1) {
                dp[i] = dp[i].max(dp[i-(k+1)] + 1);
            }
        }

        dp[n]
    }
}

fn is_palin(ch:&[u8], left:usize, right: usize) -> bool {
    let mut left = left;
    let mut right = right;
    while left < right {
        if ch[left] != ch[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {

        assert_eq!(2, Solution::max_palindromes(String::from("abaccdbbd"), 3));
    }
}
