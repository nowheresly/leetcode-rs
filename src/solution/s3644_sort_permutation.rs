pub struct Solution {}

impl Solution {
    pub fn sort_permutation(nums: Vec<i32>) -> i32 {
        let mut k = i32::MAX;

        for i in 0..nums.len() {
            let val = nums[i];
            if val != i as i32 {
                k &= val;
            }
        }
        if k != i32::MAX { k } else { 0 }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::sort_permutation(vec![0, 1, 3, 2]));
    }
}
