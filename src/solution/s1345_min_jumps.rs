use std::collections::{HashMap, VecDeque};

pub struct Solution {}

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        if n == 1 {
            return 0;
        }
        if n == 2 {
            return 1;
        }
        // value -> list of indices
        let mut map:HashMap<i32, Vec<usize>> = HashMap::new();
        for i in 0..n {
            map.entry(arr[i]).or_insert(vec![]).push(i);
        }
        let mut q:VecDeque<usize> = VecDeque::new();
        q.push_back(0);
        let mut visited = vec![false; n];
        visited[0] = true;
        let mut steps = 0;
        while q.is_empty() == false {
            let size = q.len();
            for i in 0..size {
                let curr = q.pop_front().unwrap();
                if curr == n - 1 {
                    return steps;
                }
                if let Some(wide_jumps) = map.remove(&arr[curr]) {
                    for next in wide_jumps {
                        if !visited[next] {
                            visited[next] = true;
                            q.push_back(next);
                        }
                    }
                }
                if curr + 1 < n && !visited[curr + 1] {
                    visited[curr + 1] = true;
                    q.push_back(curr + 1);
                }

                if curr > 0 && !visited[curr - 1] {
                    visited[curr - 1] = true;
                    q.push_back(curr - 1);
                }
            }
            steps += 1;
        }
        steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::min_jumps(vec![100,-23,-23,404,100,23,23,23,3,404]));
    }
}
