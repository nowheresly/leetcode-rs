

pub struct Solution {}

impl Solution {
    pub fn largest_integer(n: i32, s: i32) -> i32 {
        if n * 9 < s {
            return -1;
        }
        if s == 0 {
            return 0;
        }
        let mut res = 0;
        let mut n = n;
        let mut s = s;
        while n > 0 {
            let val = s.min(9);
            s -= val;
            res *= 10;
            res += val;
            n -= 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            90,
            Solution::largest_integer(2, 9)
        );
    }

}
