pub struct Solution {}

impl Solution {
    pub fn min_moves2(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let median = nums[nums.len() / 2];
        let mut moves = 0;
        for num in nums {
            moves += i32::abs(num - median);
        }
        moves
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            16,
            Solution::min_moves2(vec![1,10,2,9])
        );
    }
}
