

pub struct Solution {}

impl Solution {
    pub fn first_unique_even(nums: Vec<i32>) -> i32 {
        let mut freq = vec![0; 101];
        for &i in nums.iter() {
            freq[i as usize] += 1;
        }

        for i in 0..nums.len() {
            if nums[i] % 2 == 0 && freq[nums[i] as usize] == 1 {
                return nums[i];
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
            2,
            Solution::first_unique_even( vec! [3,4,2,5,4,6])
        );
    }

}
