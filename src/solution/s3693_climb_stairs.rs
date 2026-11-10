pub struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32, costs: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        dp[1] = costs[0] + 1;
        if n >= 2 {
            dp[2] = i32::min(costs[1] + 4, dp[1] + costs[1] + 1);
        }
        for i in 3..=n {
            dp[i] = dp[i - 1] + costs[i - 1] + 1;
            dp[i] = dp[i].min(dp[i - 2] + costs[i - 1] + 4);
            dp[i] = dp[i].min(dp[i - 3] + costs[i - 1] + 9);
        }
        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(11, Solution::climb_stairs(4, vec![5, 1, 6, 2]));
    }
}
