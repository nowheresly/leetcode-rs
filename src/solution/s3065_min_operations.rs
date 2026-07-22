
pub struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut res = 0;
        for i in nums {
            if i < k {
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
        assert_eq!(3, Solution::min_operations(vec![2,11,10,1,3], 10));

    }
}
