
pub struct Solution {}

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] < nums[right] {
                right = mid;
            } else if nums[mid] > nums[right] {
                left = mid + 1;
            } else {
                right -= 1;
            }
        }
        nums[left]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(0, Solution::find_min(vec![2,2,2,0,1]));

    }
}
