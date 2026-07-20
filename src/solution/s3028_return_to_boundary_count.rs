
pub struct Solution {}

impl Solution {
    pub fn return_to_boundary_count(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut pos = 0;
        for i in nums {
            pos += i;
            if pos == 0 {
                res += 1;
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
        assert_eq!(0, Solution::return_to_boundary_count(vec![3,2,-3,-4]));
    }
}
