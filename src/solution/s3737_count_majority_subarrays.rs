pub struct Solution {}

impl Solution {
    pub fn count_majority_subarrays(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut res = 0;

        for start in 0..n {
            let mut count = 0;
            for end in start..n {
                if nums[end] == target {
                    count += 1;
                }
                if 2 * count > end - start + 1 {
                    res += 1;
                }
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
        assert_eq!(5, Solution::count_majority_subarrays(vec![1,2,2,3], 2));
    }
}
