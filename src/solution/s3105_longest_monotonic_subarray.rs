
pub struct Solution {}

impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let mut ans_inc = 1;
        let mut ans_dec = 1;
        let mut ans_max_inc = 1;
        let mut ans_max_dec = 1;
        let n = nums.len();

        let mut prev = nums[0];
        for i in 1..n {
            let cur = nums[i];
            if cur > prev {
                ans_inc += 1;
                ans_max_dec = ans_max_dec.max(ans_dec);
                ans_dec = 1;
            } else if cur < prev {
                ans_max_inc = ans_max_inc.max(ans_inc);
                ans_dec += 1;
                ans_inc = 1;
            } else {
                ans_max_inc = ans_max_inc.max(ans_inc);
                ans_inc = 1;
                ans_max_dec = ans_max_dec.max(ans_dec);
                ans_dec = 1;
            }
            prev = cur;
        }
        ans_max_inc = ans_max_inc.max(ans_inc);
        ans_max_dec = ans_max_dec.max(ans_dec);
        i32::max(ans_max_inc, ans_max_dec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::longest_monotonic_subarray(vec![1,4,3,3,2]));
    }
}
