pub struct Solution {}

impl Solution {
    pub fn consecutive_set_bits(n: i32) -> bool {
        let mut prev = 0;
        let mut count = 0;
        for i in 0..32 {
            let val = (n >> i) & 1;
            if val == 1 && prev == 1 {
                count += 1;
            }
            prev = val;
        }
        count == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            false,
            Solution::consecutive_set_bits(5)
        );
    }

}
