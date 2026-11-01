pub struct Solution {}

impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let mut min = i32::MAX;
        let mut sum:i64 = 0;

        for &num in nums.iter() {
            sum += num as i64;
            min = min.min(num);
        }
        (sum - nums.len() as i64 * min as i64) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            3,
            Solution::min_moves(vec![1,2,3])
        );
    }
}
