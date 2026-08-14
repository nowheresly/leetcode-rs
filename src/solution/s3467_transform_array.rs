pub struct Solution {}

impl Solution {
    pub fn transform_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let n = nums.len();
        let mut even = 0;
        for i in 0..n {
            if nums[i] % 2 == 0 {
                even += 1;
            }
        }
        for i in 0..n {
            if even > 0 {
                nums[i] = 0;
                even -= 1;
            } else {
                nums[i] = 1;
            }
        }
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![0,0,1,1,1], Solution::transform_array(vec![1,5,1,4,2]));
    }
}
