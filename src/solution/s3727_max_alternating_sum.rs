pub struct Solution {}

impl Solution {
    pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
        let mut nums = nums;
        let n = nums.len();

        for num in nums.iter_mut() {
            *num *= *num;
        }
        nums.sort_unstable();

        let mut ans:i64 = 0;
        for i in 0..n / 2 {
            ans -= nums[i] as i64;
        }
        for i in n / 2..n {
            ans += nums[i] as i64;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(16, Solution::max_alternating_sum(vec![1,-1,2,-2,3,-3]));
    }
}
