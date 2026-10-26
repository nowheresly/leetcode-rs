pub struct Solution {}

impl Solution {
    pub fn count_islands(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut grid = grid;
        let mut res = 0;

        for r in 0..m {
            for c in 0..n {
                if grid[r][c] > 0 {
                    let island_sum = dfs(&mut grid, r, c);
                    if island_sum % (k as i64) == 0 {
                        res += 1;
                    }
                }
            }
        }
        res
    }
}

fn dfs(grid: &mut Vec<Vec<i32>>, r: usize, c: usize) -> i64 {
    
    if grid[r][c] == 0 {
        return 0;
    }
    let mut sum = grid[r][c] as i64;
    grid[r][c] = 0;
    let directions: [(i32, i32); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];
    for (dr, dc) in directions {
        if (r == 0 && dr == -1) || (c == 0 && dc == -1) {
            continue;
        }
        let nr = (r as i32 + dr) as usize;
        let nc = (c as i32 + dc) as usize;
        if nc >= grid[0].len() || nr >= grid.len() || grid[nr][nc] == 0 {
            continue;
        }
        sum += dfs(grid, nr, nc);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            2,
            Solution::count_islands(
                vec![
                    vec![0, 2, 1, 0, 0],
                    vec![0, 5, 0, 0, 5],
                    vec![0, 0, 1, 0, 0],
                    vec![0, 1, 4, 7, 0],
                    vec![0, 2, 0, 0, 8]
                ],
                5
            )
        );
    }
}
