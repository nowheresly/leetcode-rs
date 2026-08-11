pub struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let n = nums.len();
        let mut i = 0;
        while i < n {
            while i != n-1 && nums[i+1] == nums[i] {
                i += 1;
            }
            i += 1;
            res += 1;
        }
        res - 1
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::min_operations(vec![1,4,2]));

    }
}