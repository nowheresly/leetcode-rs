pub struct Solution {}

impl Solution {
    pub fn minimum_costs(regular: Vec<i32>, express: Vec<i32>, express_cost: i32) -> Vec<i64> {
        let n = regular.len();
        let express_cost = express_cost as i64;
        let mut dp: Vec<Vec<i64>> = vec![vec![0; 2]; n + 1];
        dp[0][1] = express_cost;

        for i in 1..=n {
            dp[i][0] =
                (dp[i - 1][0] + regular[i - 1] as i64).min(dp[i - 1][1] + express[i - 1] as i64);
            dp[i][1] = (dp[i][0] + express_cost).min(dp[i - 1][1] + express[i - 1] as i64);
        }

        let mut ans = vec![0; n];
        for i in 1..=n {
            ans[i - 1] = dp[i][0].min(dp[i][1]);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![10, 15, 24],
            Solution::minimum_costs(vec![11, 5, 13], vec![7, 10, 6], 3)
        );
    }
}
