

pub struct Solution {}

impl Solution {
    pub fn xor_after_queries(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let mut nums = nums;
        let mut res = 0;
        for q in queries {
            let mut idx = q[0];
            while idx <= q[1] {
                nums[idx as usize] = ((nums[idx as usize] as i64 * q[3] as i64) % 1_000_000_007i64) as i32;
                idx += q[2];
            }
        }
        for i in nums {
            res ^= i;
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
            4,
            Solution::xor_after_queries(vec![1,1,1],  vec![vec![0,2,1,4]])
        );
    }

}
