pub struct Solution {}

impl Solution {
    pub fn max_jumps(arr: Vec<i32>, d: i32) -> i32 {
        let n = arr.len() as i32;
        let mut dp = vec![0; n as usize];
        let mut max = 0;

        for i in 0..n {
            max = max.max(dfs(&arr, &mut dp, i, d));
        }
        max
    }
}

fn dfs(arr: &Vec<i32>, dp: &mut Vec<i32>, start: i32, d: i32) -> i32 {
    if dp[start as usize] != 0 {
        return dp[start as usize];
    }

    let n = arr.len() as i32;
    let mut res = 1;

    for i in start + 1..=i32::min(n-1, start + d) {
        if arr[i as usize] >= arr[start as usize] {
            break;
        }
        res = res.max(1 + dfs(arr, dp, i, d));
    }
    for i in (i32::max(0, start - d)..=start - 1).rev() {
        if arr[i as usize] >= arr[start as usize] {
            break;
        }
        res = res.max(1 + dfs(arr, dp, i, d));
    }
    dp[start as usize] = res;
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            4,
            Solution::max_jumps(vec![6,4,14,6,8,13,9,7,10,6,12], 2)
        );

    }
}
