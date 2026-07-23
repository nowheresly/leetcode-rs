pub struct Solution {}

impl Solution {
    pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 2 {
            return n as i32;
        }
        let mut res = 1;
        while res <= n {
            res <<= 1;
        }
        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::unique_xor_triplets(vec![3,1,2]));
    }


}
