use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn find_safe_walk(grid: Vec<Vec<i32>>, health: i32) -> bool {
        let m = grid.len();
        let n = grid[0].len();
        let mut q = VecDeque::new();
        q.push_back((0, 0));
        let mut qnext = VecDeque::new();
        let mut cur = grid[0][0];
        if cur >= health {
            return false;
        }
        let mut h = vec![vec![-1; n]; m];
        while !q.is_empty() || !qnext.is_empty() {
            if q.is_empty() {
                q = qnext;
                qnext = VecDeque::new();
                cur += 1;
                if cur >= health {
                    return false;
                }
                continue;
            }
            let (x, y) = q.pop_front().unwrap();
            if h[x][y] != -1 {
                continue;
            }
            if x == m-1 && y == n-1 {
                return true;
            }
            h[x][y] = cur;
            for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 && h[nx as usize][ny as usize] == -1 {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if grid[nx][ny] == 0 {
                        q.push_back((nx, ny));
                    } else {
                        qnext.push_back((nx, ny));
                    }
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::find_safe_walk(vec![vec![0,1,0,0,0],vec![0,1,0,1,0],vec![0,0,0,1,0]], 1));
    }
}
