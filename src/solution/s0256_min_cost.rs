pub struct Solution {}

impl Solution {
    pub fn min_cost(costs: Vec<Vec<i32>>) -> i32 {
        let n = costs.len();

        let mut dp = vec![vec![0;3];n];
        dp[0][0] = costs[0][0];
        dp[0][1] = costs[0][1];
        dp[0][2] = costs[0][2];

        for i in 1..n {
            dp[i][0] = costs[i][0] + dp[i-1][1].min(dp[i-1][2]);
            dp[i][1] = costs[i][1] + dp[i-1][0].min(dp[i-1][2]);
            dp[i][2] = costs[i][2] + dp[i-1][0].min(dp[i-1][1]);
        }

        dp[n-1][0].min(dp[n-1][1]).min(dp[n-1][2])
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(10, Solution::min_cost(vec! [vec![17,2,17],vec![16,16,5],vec![14,3,19]]));
    }
}
