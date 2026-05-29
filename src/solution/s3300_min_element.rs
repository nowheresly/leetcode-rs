pub struct Solution {}

impl Solution {
    pub fn min_element(nums: Vec<i32>) -> i32 {
        let mut min = i32::MAX;
        for num in nums {
            let mut num = num;
            let mut val = 0;
            while num > 0 {
                val += num % 10;
                num /= 10;
            }
            if val < min {
                min = val;
            }
        }
        min
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::min_element(vec![10,12,13,14]));
    }
}