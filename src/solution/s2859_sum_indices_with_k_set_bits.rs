
pub struct Solution {}

impl Solution {
    pub fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32 {
        let mut res = 0;
        for i in 0..nums.len() {
            let mut bits = usize::count_ones(i) as i32;
            if bits == k {
                res += nums[i];
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(13, Solution::sum_indices_with_k_set_bits(vec![5,10,1,5,2], 1));

    }
}
