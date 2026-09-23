pub struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let mut total_sum = 0;
        for num in nums.iter() {
            total_sum += num;
        }

        let mut target = total_sum - x;
        if target < 0 {
            return -1;
        }
        if target == 0 {
            return nums.len() as i32;
        }

        let mut left = 0;
        let mut current_sum = 0;
        let mut max_len: i32 = -1;

        for right in 0..nums.len() {
            current_sum += nums[right];

            while (left <= right && current_sum > target) {
                current_sum -= nums[left];
                left += 1;
            }

            if (current_sum == target) {
                max_len = max_len.max(right as i32 - left as i32 + 1);
            }
        }

        if max_len == -1 {
            return -1;
        }
        nums.len() as i32 - max_len
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(5, Solution::min_operations(vec![3,2,20,1,1,3], 10));
    }
}
