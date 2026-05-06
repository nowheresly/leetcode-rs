pub struct Solution {}

impl Solution {
    pub fn rotate_the_box(box_grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let m = box_grid.len();
        let n = box_grid[0].len();

        let mut res = vec![vec!['.'; m]; n];

        for i in 0..m {
            let mut bottom: isize = (n - 1) as isize;

            for j in (0..n).rev() {
                if box_grid[i][j] == '*' {
                    res[j][m - 1 - i] = '*';
                    bottom = j as isize - 1;
                } else if box_grid[i][j] == '#' {
                    res[bottom as usize][m - 1 - i] = '#';
                    bottom -= 1;
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
            vec![
                vec!['.', '#', '#'],
                vec!['.', '#', '#'],
                vec!['#', '#', '*'],
                vec!['#', '*', '.'],
                vec!['#', '.', '*'],
                vec!['#', '.', '.']
            ],
            Solution::rotate_the_box(vec![
                vec!['#', '#', '*', '.', '*', '.'],
                vec!['#', '#', '#', '*', '.', '.'],
                vec!['#', '#', '#', '.', '#', '.']
            ])
        );
    }
}
