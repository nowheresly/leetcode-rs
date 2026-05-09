pub struct Solution {}

impl Solution {
    pub fn rotate_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let mut res = vec![vec![0; n]; m];

        for layer in 0..(usize::min(m, n) / 2) {
            let mut buffer = vec![0; 2 * (2 * (m + n - 4 * layer) - 4)];
            let mut index = 0;
            // copy top line
            for i in layer..n - layer {
                buffer[index] = grid[layer][i];
                index += 1;
            }
            // copy right column
            for i in layer + 1..m - layer {
                buffer[index] = grid[i][n - layer - 1];
                index += 1;
            }
            // copy bottom line
            for i in (layer..n - layer - 1).rev() {
                buffer[index] = grid[m - layer - 1][i];
                index += 1;
            }
            // copy left column
            for i in (layer + 1..m - layer - 1).rev() {
                buffer[index] = grid[i][layer];
                index += 1;
            }
            // rotate buffer
            for i in 0..index {
                buffer[i + index] = buffer[i];
            }
            let mut rot = k as usize % index;
            // copy top line
            for i in layer..n - layer {
                res[layer][i] = buffer[rot];
                rot += 1;
            }
            // copy right column
            for i in layer + 1..m - layer {
                res[i][n - layer - 1] = buffer[rot];
                rot += 1;
            }
            // copy bottom
            for i in (layer..n - layer - 1).rev() {
                res[m - layer - 1][i] = buffer[rot];
                rot += 1;
            }
            // copy left column
            for i in (layer..m - layer - 1).rev() {
                res[i][layer] = buffer[rot];
                rot += 1;
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
                vec![3, 4, 8, 12],
                vec![2, 11, 10, 16],
                vec![1, 7, 6, 15],
                vec![5, 9, 13, 14]
            ],
            Solution::rotate_grid(
                vec![
                    vec![1, 2, 3, 4],
                    vec![5, 6, 7, 8],
                    vec![9, 10, 11, 12],
                    vec![13, 14, 15, 16]
                ],
                2
            )
        );
    }
}
