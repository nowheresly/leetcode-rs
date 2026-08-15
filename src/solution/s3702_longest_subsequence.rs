pub struct Solution {}

impl Solution {
    pub fn longest_subsequence(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;

        let mut all_zeroes = true;
        let mut xor = 0;
        for &i in nums.iter() {
            if i != 0 {
                all_zeroes = false;
            }
            xor ^= i;
        }
        if all_zeroes {
            return 0;
        }
        if xor != 0 {
            return n;
        }
        n - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::longest_subsequence(vec![2, 3, 4]));
    }
}
