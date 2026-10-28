pub struct Solution {}

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = vec![vec![vec![false; 1024]; n]; m];
        dp[0][0][grid[0][0] as usize] = true;
        for i in 0..m {
            for j in 0..n {
                if i == 0 && j == 0 {
                    continue;
                }
                let val = grid[i][j] as usize;
                if i > 0 {
                    for v in 0..1024 {
                        if dp[i - 1][j][v] {
                            dp[i][j][v ^ val] = true;
                        }
                    }
                }
                if j > 0 {
                    for v in 0..1024 {
                        if dp[i][j - 1][v] {
                            dp[i][j][v ^ val] = true;
                        }
                    }
                }
            }
        }
        for v in 0..1024 {
            if dp[m - 1][n - 1][v] {
                return v as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(9, Solution::min_cost(vec![vec![6, 7], vec![5, 8]]));
    }
}
