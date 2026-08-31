pub struct Solution {}

impl Solution {
    pub fn limit_occurrences(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut write = 0;
        let k = k as usize;
        let mut nums = nums;
        for read in 0..nums.len() {
            if write < k || nums[read] != nums[write - k] {
                nums[write] = nums[read];
                write += 1;
            }
        }
        nums.truncate(write);
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![1,1,2,2,3],
            Solution::limit_occurrences(vec![1,1,1,2,2,3], 2)
        );
    }

}
