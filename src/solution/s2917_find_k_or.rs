pub struct Solution {}

impl Solution {
    pub fn find_k_or(nums: Vec<i32>, k: i32) -> i32 {
        let mut res = 0;
        for i in 0..32 {
            let mut count = 0;
            for &val in nums.iter() {
                if ((val >> i) & 1) == 1 {
                    count += 1;
                }
            }
            if count >= k {
                res += i32::pow(2, i);
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
        assert_eq!(9, Solution::find_k_or(vec![7,12,9,8,9,15], 4 ));

    }
}
