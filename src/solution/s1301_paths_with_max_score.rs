pub struct Solution {}

impl Solution {
    pub fn paths_with_max_score(board: Vec<String>) -> Vec<i32> {
        let modu = 1_000_000_007;
        let n = board.len();

        let mut max_sum = vec![vec![0; n + 1]; n + 1];
        let mut paths = vec![vec![0; n + 1]; n + 1];

        paths[n - 1][n - 1] = 1;

        for r in (0..n).rev() {
            for c in (0..n).rev() {
                let ch = board[r].as_bytes()[c] as char;
                if ch == 'X' || (r == n - 1 && c == n - 1) {
                    continue;
                }
                let max = max_sum[r + 1][c]
                    .max(max_sum[r][c + 1])
                    .max(max_sum[r + 1][c + 1]);

                let mut ways = 0;
                if max == max_sum[r + 1][c] {
                    ways = (ways + paths[r + 1][c]) % modu;
                }
                if max == max_sum[r][c + 1] {
                    ways = (ways + paths[r][c + 1]) % modu;
                }
                if max == max_sum[r + 1][c + 1] {
                    ways = (ways + paths[r + 1][c + 1]) % modu;
                }
                if ways > 0 {
                    paths[r][c] = ways;
                    let val = if ch == 'E' { 0 } else { ch as u8 - '0' as u8 };
                    let val = val as i32;
                    max_sum[r][c] = max + val;
                }
            }
        }
        vec![
            if paths[0][0] == 0 { 0 } else { max_sum[0][0] },
            paths[0][0],
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ret = Solution::paths_with_max_score(vec![
            "E23".to_string(),
            "2X2".to_string(),
            "12S".to_string(),
        ]);
        assert_eq!(vec![7, 1], ret);
    }
}
