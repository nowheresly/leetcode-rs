pub struct Solution {}

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let x = nums[nums.len() - 1];
        let y = nums[nums.len() - 2];
        (x - 1) * (y - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(16, Solution::max_product(vec![1, 5, 4, 5]));
    }
}
