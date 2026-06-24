pub struct Solution {}

impl Solution {
    pub fn can_alice_win(n: i32) -> bool {
        let mut alice = false;
        let mut last = 10;
        let mut n = n;
        while n >= last {
            n -= last;
            last -= 1;
            alice = ! alice;
        }
        alice
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::can_alice_win(12));
    }
}
