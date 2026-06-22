pub struct Solution {}

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == 0 {
                    continue;
                }
                // top
                if i == 0 {
                    res += 1;
                } else if grid[i - 1][j] == 0 {
                    res += 1;
                }
                // left
                if j == 0 {
                    res += 1;
                } else if grid[i][j - 1] == 0 {
                    res += 1;
                }
                // right
                if j == grid[i].len() - 1 {
                    res += 1;
                } else if grid[i][j + 1] == 0 {
                    res += 1;
                }
                // bottom
                if i == grid.len() - 1 {
                    res += 1;
                } else if grid[i + 1][j] == 0 {
                    res += 1;
                }
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            16,
            Solution::island_perimeter(vec![
                vec![0, 1, 0, 0],
                vec![1, 1, 1, 0],
                vec![0, 1, 0, 0],
                vec![1, 1, 0, 0]
            ])
        );
    }
}
