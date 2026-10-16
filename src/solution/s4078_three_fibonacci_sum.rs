pub struct Solution {}

impl Solution {
    pub fn three_fibonacci_sum(n: i32) -> bool {
        let mut a = 0;
        let mut b = 1;
        let mut c = 1;
        while a + b + c < n {
            a = b;
            b = c;
            c = a + b;
        }
        a + b + c == n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(false, Solution::three_fibonacci_sum(8));
    }
}
