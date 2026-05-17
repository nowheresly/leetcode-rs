
pub struct Solution {}

impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        dfs(&arr, start, &mut vec![false; arr.len()])
    }
}

fn dfs(arr: &Vec<i32>, start: i32, visited: &mut Vec<bool>) -> bool {
    if start < 0 || start >= arr.len() as i32 || visited[start as usize] {
        return false;
    }
    if arr[start as usize] == 0 {
        return true;
    }

    visited[start as usize] = true;

    dfs(arr, start + arr[start as usize], visited) || dfs(arr, start - arr[start as usize], visited)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {

        assert_eq!(true, Solution::can_reach(vec![4,2,3,0,3,1,2], 5));

    }
}
