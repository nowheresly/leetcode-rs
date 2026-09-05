

pub struct Solution {}

impl Solution {
    pub fn first_stable_index(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut min = vec![0; n];
        min[n - 1] = nums[n - 1];
        for i in (0..n - 1).rev() {
            min[i] = min[i + 1].min(nums[i]);
        }
        let mut max = i32::MIN;
        for i in 0..n {
            max = max.max(nums[i]);
            if max - min[i] <= k {
                return i as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            3,
            Solution::first_stable_index(vec![5,0,1,4], 3)
        );
    }

}
