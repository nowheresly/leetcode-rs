pub struct Solution {}

impl Solution {
    pub fn sum_base(n: i32, k: i32) -> i32 {
        let mut sum = 0;
        let mut n = n;
        while n > 0 {
            sum += n % k;
            n /= k;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(9, Solution::sum_base(34, 6));

    }
}
