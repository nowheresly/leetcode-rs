pub struct Solution {}

impl Solution {
    pub fn k_items_with_maximum_sum(num_ones: i32, num_zeros: i32, _num_neg_ones: i32, k: i32) -> i32 {
        if num_ones + num_zeros >= k {
            return k.min(num_ones);
        }
        num_ones + num_zeros - 0.max(k - num_ones)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::k_items_with_maximum_sum(3,2,0,2));

    }
}