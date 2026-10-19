
pub struct Solution {}

impl Solution {
    pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums = nums;
        nums.sort();
        let mut res = vec![];
        for i in 0..nums.len() {
            if nums[i] == target {
                res.push(i as i32);
            } else if nums[i] > target {
                break;
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

        assert_eq!(vec![1,2], Solution::target_indices(vec![1,2,5,2,3], 2));

    }
}
