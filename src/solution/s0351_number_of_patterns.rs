pub struct Solution {}

impl Solution {
    pub fn number_of_patterns(m: i32, n: i32) -> i32 {
        let mut jumps = vec![vec![0; 10]; 10];
        jumps[1][3] = 2;
        jumps[3][1] = 2;
        jumps[4][6] = 5;
        jumps[6][4] = 5;
        jumps[7][9] = 8;
        jumps[9][7] = 8;
        jumps[1][7] = 4;
        jumps[7][1] = 4;
        jumps[2][8] = 5;
        jumps[8][2] = 5;
        jumps[3][9] = 6;
        jumps[9][3] = 6;
        jumps[1][9] = 5;
        jumps[9][1] = 5;
        jumps[3][7] = 5;
        jumps[7][3] = 5;

        let mut res = 0;
        for k in m..=n {
            for i in 1..=9 {
                let mut visited = vec![false; 10];
                res += dfs(&jumps, &mut visited, i, k);
            }
        }
        res
    }
}

fn dfs(jumps: &Vec<Vec<usize>>, visited: &mut Vec<bool>, current_dot:usize, remaining_steps:i32) -> i32 {
    if remaining_steps == 1 {
        return 1;
    }
    visited[current_dot] = true;
    let mut sum = 0;
    for next in 1..=9 {
        if visited[next] {
            continue;
        }
        let jump_dot = jumps[current_dot][next];
        if jump_dot != 0 && visited[jump_dot] == false {
            // invalid move
            continue;
        }
        sum += dfs(jumps, visited, next, remaining_steps - 1);
    }
    visited[current_dot] = false;
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(65, Solution::number_of_patterns(1, 2));
    }
}
