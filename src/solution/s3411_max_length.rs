pub struct Solution {}

impl Solution {
    pub fn max_length(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut res = 0;
        for i in 0..n {
            let mut curr_gcd = nums[i];
            let mut curr_lcm = nums[i];
            let mut curr_pro = nums[i];
            for j in (i + 1)..n {
                curr_pro *= nums[j];
                curr_gcd = gcd(curr_gcd, nums[j]);
                curr_lcm = lcm(curr_lcm, nums[j]);

                if curr_pro == curr_gcd * curr_lcm {
                    res = res.max(j as i32 - i as i32 + 1)
                }
            }
        }
        res
    }
}

fn lcm(a: i32, b: i32) -> i32 {
    (a / gcd(a, b)) * b
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a } else { gcd(b, a % b) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums: Vec<i32> = vec![1, 2, 1, 2, 1, 1, 1];
        assert_eq!(5, Solution::max_length(nums));
    }
}
