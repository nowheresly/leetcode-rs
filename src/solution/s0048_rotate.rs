pub struct Solution {}
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();

        // swap top bottom
        matrix.reverse();
        // transpose
        for row in 0..n {
            for col in 0..row {
                if col == row {
                    continue;
                }
                let tmp = matrix[row][col];
                matrix[row][col] = matrix[col][row];
                matrix[col][row] = tmp;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut res = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        Solution::rotate(&mut res);
        let expected = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]];
        assert_eq!(
            expected,
            res
        );
    }
}
