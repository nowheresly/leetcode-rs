

pub struct Solution {}

impl Solution {
    pub fn gcd_sum(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut mx = vec![0; n];
        mx[0] = nums[0];
        let mut prefix_gcd = vec![0; n];
        prefix_gcd[0] = nums[0];
        for i in 1..n {
            mx[i] = mx[i - 1].max(nums[i]);
            prefix_gcd[i] = gcd(mx[i], nums[i]);
        }
        prefix_gcd.sort();
        let mut res = 0;
        for i in 0..n/2 {
            res += gcd(prefix_gcd[i], prefix_gcd[n - i - 1]) as i64;
        }
        res
    }
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            5,
            Solution::gcd_sum(vec![3,6,2,8])
        );
    }

}
