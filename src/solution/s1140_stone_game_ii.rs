pub struct Solution {}

impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let n = piles.len();
        let mut prefix = vec![0; n + 1];
        for i in 0..n {
            prefix[i + 1] = prefix[i] + piles[i];
        }

        let mut dp = vec![vec![0; 2 * n]; n];

        for i in (0..n).rev() {
            let remain = prefix[n] - prefix[i];

            for m in 1..=n {
                let bound = i + 2 * m;
                if bound >= n {
                    dp[i][m] = dp[i][m].max(remain);
                    continue;
                }
                for x in 1..=(2*m) {
                    dp[i][m] = dp[i][m].max(remain - dp[i + x][m.max(x)]);
                }
            }
        }
        dp[0][1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(104, Solution::stone_game_ii(vec![1, 2, 3, 4, 5, 100]));
    }
}
