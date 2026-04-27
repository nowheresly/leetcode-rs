use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<i32>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();
        let mut q = VecDeque::new();
        q.push_back((0, 0));
        let mut visited = vec![vec![false; n]; m];
        visited[0][0] = true;
        while !q.is_empty() {
            let (x, y) = q.pop_front().unwrap();
            if x == m - 1 && y == n - 1 {
                return true;
            }
            for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx < 0 || nx >= m as i32 || ny < 0 || ny >= n as i32 {
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;
                if visited[nx][ny] {
                    continue;
                }
                if connected(grid[x][y], grid[nx][ny], (dx, dy)) {
                    visited[nx][ny] = true;
                    q.push_back((nx, ny));
                }
            }
        }
        false
    }
}

fn connected(cur: i32, next: i32, mov: (i32, i32)) -> bool {
    if mov == (1, 0) {
        if (cur == 2 || cur == 3 || cur == 4) && (next == 2 || next == 5 || next == 6) {
            return true;
        }
        return false;
    } else if mov == (0, -1) {
        //dir = 'W';
        if (cur == 1 || cur == 3 || cur == 5) && (next == 1 || next == 4 || next == 6) {
            return true;
        }
        return false;
    } else if mov == (-1, 0) {
        //dir = 'N';
        if (cur == 2 || cur == 5 || cur == 6) && (next == 2 || next == 3 || next == 4) {
            return true;
        }
    } else {
        //dir = 'E';
        if (cur == 1 || cur == 4 || cur == 6) && (next == 1 || next == 3 || next == 5) {
            return true;
        }
        return false;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::has_valid_path(vec![vec![2,4,3],vec![6,5,2]]));
    }
}
