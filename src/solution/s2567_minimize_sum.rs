pub struct Solution {}

impl Solution {
    pub fn minimize_sum(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        nums.sort();
        let a = nums[n - 1] - nums[2];
        let b = nums[n - 2] - nums[1];
        let c = nums[n - 3] - nums[0];
        a.min(b).min(c)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums: Vec<i32> = vec![1, 4, 7, 8, 5];
        assert_eq!(3, Solution::minimize_sum(nums));

    }
}