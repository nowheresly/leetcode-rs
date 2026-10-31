pub struct Solution {}

impl Solution {
    pub fn unique_paths(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let modus = 1_000_000_007;
        let mut from_left = vec![vec![0i64; n + 1]; m + 1];
        let mut from_top = vec![vec![0i64; n + 1]; m + 1];

        for i in (0..=m - 1).rev() {
            for j in (0..=n - 1).rev() {
                if i == m - 1 && j == n - 1 {
                    from_left[i][j] = 1;
                    from_top[i][j] = 1;
                    continue;
                }

                if grid[i][j] == 0 {
                    let ways = (from_left[i][j + 1] + from_top[i + 1][j]) % modus;
                    from_left[i][j] = ways;
                    from_top[i][j] = ways;
                } else {
                    from_left[i][j] = from_top[i + 1][j];
                    from_top[i][j] = from_left[i][j + 1];
                }
            }
        }

        from_left[0][0] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            5,
            Solution::unique_paths(vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 0, 0]])
        );
    }
}
