pub struct Solution {}

impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        for &i in nums.iter() {
            max = max.max(i);
        }
        let mut res = 0;
        for &i in nums.iter() {
            res += max - i;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::min_moves(vec![4,4,5]));
    }
}
