pub struct Solution {}

impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut min = 0;
        for i in nums.iter() {
            sum += i;
            min = min.min(sum);
        }
        if min > 0 {
            return 1;
        }
        i32::abs(min) + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(5, Solution::min_start_value(vec![-3,2,-3,4,2]));

    }
}
