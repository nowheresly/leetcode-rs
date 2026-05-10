pub struct Solution {}

impl Solution {
    pub fn maximum_jumps(nums: Vec<i32>, target: i32) -> i32 {
        let target = i32::abs(target);
        let n = nums.len();
        let mut dp = vec![-1; n];
        dp[n - 1] = 0;
        for i in (0..n - 1).rev() {
            for j in i + 1..n {
                if dp[j] == -1 {
                    continue;
                }
                let diff = i32::abs(nums[j] - nums[i]);
                if diff > target {
                    continue;
                }
                dp[i] = dp[i].max(dp[j] + 1);
            }
        }
        dp[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(5, Solution::maximum_jumps(vec![1, 3, 6, 4, 1, 2], 3));
    }
}
