pub struct Solution {}

impl Solution {
    pub fn candy_crush(board: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = board.len();
        let n = board[0].len();
        let mut board = board;
        let mut moved = true;
        while moved {
            moved = false;

            for r in 0..m {
                for c in 0..n {
                    let val = board[r][c].abs();
                    if val == 0 {
                        continue;
                    }
                    if c + 2 < n && board[r][c + 1].abs() == val && board[r][c + 2].abs() == val {
                        board[r][c] = -val;
                        board[r][c+1] = -val;
                        board[r][c+2] = -val;
                        moved = true;
                    }

                    if r + 2 < m && board[r+1][c].abs() == val && board[r+2][c].abs() == val {
                        board[r][c] = -val;
                        board[r+1][c] = -val;
                        board[r+2][c] = -val;
                        moved = true;
                    }
                }
            }
            if !moved {
                continue;
            }
            for c in 0..n {
                let mut write_row = m;
                for r in (0..m).rev() {
                    if board[r][c] > 0 {
                        write_row -= 1;
                        board[write_row][c] = board[r][c];
                    }
                }
                for r in 0..write_row {
                    board[r][c] = 0;
                }
            }
        }
        board
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0],
                vec![110, 0, 0, 0, 114],
                vec![210, 0, 0, 0, 214],
                vec![310, 0, 0, 113, 314],
                vec![410, 0, 0, 213, 414],
                vec![610, 211, 112, 313, 614],
                vec![710, 311, 412, 613, 714],
                vec![810, 411, 512, 713, 1014]
            ],
            Solution::candy_crush(vec![
                vec![110, 5, 112, 113, 114],
                vec![210, 211, 5, 213, 214],
                vec![310, 311, 3, 313, 314],
                vec![410, 411, 412, 5, 414],
                vec![5, 1, 512, 3, 3],
                vec![610, 4, 1, 613, 614],
                vec![710, 1, 2, 713, 714],
                vec![810, 1, 2, 1, 1],
                vec![1, 1, 2, 2, 2],
                vec![4, 1, 4, 4, 1014]
            ])
        );
    }
}
