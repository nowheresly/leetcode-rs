pub struct Solution {}

impl Solution {
    pub fn stone_game_v(stone_value: Vec<i32>) -> i32 {
        let n = stone_value.len();
        if n == 1 {
            return 0;
        }

        let mut prefix = vec![0; n + 1];
        for i in 0..n {
            prefix[i + 1] = prefix[i] + stone_value[i];
        }

        let mut dp = vec![vec![0; n]; n];
        let mut max_l = vec![vec![0; n]; n];
        let mut max_r = vec![vec![0; n]; n];

        for i in 0..n {
            max_l[i][i] = stone_value[i];
            max_r[i][i] = stone_value[i];
        }

        // 3. Iterative DP (Bottom-Up)
        // i moves backwards, j moves forwards to build subarrays
        for i in (0..n).rev() {
            let mut left_end = i;

            for j in i + 1..n {
                let total_sum = prefix[j + 1] - prefix[i];

                // Advance 'm' as long as left sum <= right sum.
                // leftSum = prefix[m+2] - prefix[i]
                // leftSum <= rightSum is mathematically identical to: leftSum * 2 <= totalSum
                while left_end < j && (prefix[left_end + 1] - prefix[i]) * 2 <= total_sum {
                    left_end += 1;
                }

                let mut res = 0;

                if left_end > i {
                    res = res.max(max_l[i][left_end - 1]);
                }

                if left_end > i && (prefix[left_end] - prefix[i]) * 2 == total_sum {
                    res = res.max(max_r[left_end][j]);
                }

                if left_end + 1 <= j {
                    res = res.max(max_r[left_end + 1][j]);
                }

                dp[i][j] = res;

                max_l[i][j] = max_l[i][j - 1].max(dp[i][j] + total_sum);
                max_r[i][j] = max_r[i + 1][j].max(dp[i][j] + total_sum);
            }
        }
        dp[0][n - 1]
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(18, Solution::stone_game_v(vec![6, 2, 3, 4, 5, 5]));
    }
}
