pub struct Solution {}

impl Solution {
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut memo = vec![vec![-1; n]; n];

        predict(&nums, 0, n as i32 - 1, &mut memo) >= 0
    }
}

fn predict(nums: &[i32], l: i32, r: i32, memo: &mut Vec<Vec<i32>>) -> i32 {
    if l > r {
        return 0;
    }
    if memo[l as usize][r as usize] != -1 {
        return memo[l as usize][r as usize];
    }
    // case first
    let ret_first = nums[l as usize] - predict(nums, l+1, r, memo);

    // case last
    let ret_last = nums[r as usize] - predict(nums, l, r-1, memo);

    memo[l as usize][r as usize] = ret_first.max(ret_last);
    memo[l as usize][r as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            true,
            Solution::predict_the_winner(
                vec![1,5,233,7]
            )
        );
    }
}
