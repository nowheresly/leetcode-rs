
pub struct Solution {}

impl Solution {
    pub fn has_trailing_zeros(nums: Vec<i32>) -> bool {
        let mut count = 0;
        for i in nums {
            if i & 1 == 0 {
                count += 1;
                if count > 1 {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(false, Solution::has_trailing_zeros(vec![1,3,5,7,9]));
    }

    #[test]
    fn test_2() {
        assert_eq!(true, Solution::has_trailing_zeros(vec![2,4,6,8,10]));
    }
}
