pub struct Solution {}

impl Solution {
    pub fn smallest_number(n: i32, t: i32) -> i32 {
        let mut res = n.max(t);
        loop {
            let mut val = res;
            let mut prod = 1;
            while val > 0 {
                let digit = val % 10;
                if digit == 0 {
                    return res;
                }
                prod *= digit;
                val /= 10;
            }
            if prod % t == 0 {
                return res;
            }
            res += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(16, Solution::smallest_number(15, 3));
    }
}
