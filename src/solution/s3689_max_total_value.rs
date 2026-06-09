pub struct Solution {}

impl Solution {
    pub fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
        let m1 = *nums.iter().min().unwrap() as i64;
        let m2 = *nums.iter().max().unwrap() as i64;
        (m2 - m1) * k as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(12, Solution::max_total_value(vec![4,2,5,1], 3));

    }
}
