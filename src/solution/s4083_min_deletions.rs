pub struct Solution {}

impl Solution {
    pub fn min_deletions(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for i in 1..nums.len() {
            if nums[i] == nums[i-1] {
                res += 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::min_deletions(vec![1,1,2,2,2,3]));
    }
}
