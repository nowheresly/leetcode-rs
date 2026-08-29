pub struct Solution {}

impl Solution {
    pub fn count_commas(n: i32) -> i32 {
        let mut res = 0;
        let mut base = 1000;

        while n >= base {
            res += n - base + 1;
            base *= 1000;
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
            9069,
            Solution::count_commas(10068)
        );
    }

}
