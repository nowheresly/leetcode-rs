pub struct Solution {}

impl Solution {
    pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
        let n = stone_value.len();
        let mut dp = vec![0; n+1];
        for i in (0..n).rev() {
            dp[i] = stone_value[i] - dp[i+1];
            if i +2 <= n {
                dp[i] = dp[i].max(stone_value[i]+stone_value[i+1] - dp[i+2]);
            }
            if i + 3 <= n {
                dp[i] = dp[i].max(stone_value[i]+stone_value[i+1]+stone_value[i+2] - dp[i+3]);
            }
        }
        if dp[0] > 0 {
            return "Alice".to_string();
        }
        if dp[0] < 0 {
            return "Bob".to_string();
        }
        "Tie".to_string()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(String::from("Alice"), Solution::stone_game_iii(vec![1,2,3,-9]));

    }
}
