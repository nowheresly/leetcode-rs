pub struct Solution {}

impl Solution {
    pub fn even_odd_bit(n: i32) -> Vec<i32> {
        let mut even = 0;
        let mut odd = 0;

        for i in 0..32 {
            if ((n >> i) & 1) == 1 {
                if (i & 1) == 1 {
                    odd += 1;
                } else {
                    even += 1;
                }
            }
        }

        vec![even, odd]
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![1, 2], Solution::even_odd_bit(50));

    }
}