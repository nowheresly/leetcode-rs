pub struct Solution {}

impl Solution {
    pub fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut grid = grid;

        // every land close to the border should be ignored
        for row in 0..n {
            if grid[row][0] == 0 {
                dfs(&mut grid, row, 0);
            }
            if grid[row][m - 1] == 0 {
                dfs(&mut grid, row, m - 1);
            }
        }
        // every land close to the border should be ignored
        for col in 0..m {
            if grid[0][col] == 0 {
                dfs(&mut grid, 0, col);
            }
            if grid[n - 1][col] == 0 {
                dfs(&mut grid, n - 1, col);
            }
        }

        let mut res = 0;
        for row in 1..(n - 1) {
            for col in 1..(m - 1) {
                if grid[row][col] == 0 {
                    dfs(&mut grid, row, col);
                    res += 1;
                }
            }
        }
        res
    }
}

fn dfs(grid: &mut Vec<Vec<i32>>, row: usize, col: usize) {
    if row >= grid.len() || col >= grid[0].len() || grid[row][col] == 1 {
        return;
    }
    grid[row][col] = 1;
    if row > 0 {
        dfs(grid, row - 1, col);
    }
    dfs(grid, row + 1, col);
    dfs(grid, row, col + 1);
    if col > 0 {
        dfs(grid, row, col - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            2,
            Solution::closed_island(vec![
                vec![1, 1, 1, 1, 1, 1, 1, 0],
                vec![1, 0, 0, 0, 0, 1, 1, 0],
                vec![1, 0, 1, 0, 1, 1, 1, 0],
                vec![1, 0, 0, 0, 0, 1, 0, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 0]
            ])
        );
    }
}
