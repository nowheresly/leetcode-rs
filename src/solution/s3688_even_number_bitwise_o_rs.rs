pub struct Solution {}

impl Solution {
    pub fn even_number_bitwise_o_rs(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for i in nums.iter() {
            if i % 2 == 0 {
                res |= i;
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
        assert_eq!(6, Solution::even_number_bitwise_o_rs(vec![1,2,3,4,5,6]));
    }
}
