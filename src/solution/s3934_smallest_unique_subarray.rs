pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn smallest_unique_subarray(nums: Vec<i32>) -> i32 {
        const MOD: i128 = 1_000_000_007;
        const BASE: i128 = 911_382_323;

        let n = nums.len();
        if n == 0 {
            return 0;
        }

        let mut h = vec![0i128; n + 1];
        let mut p = vec![0i128; n + 1];
        p[0] = 1;

        for (i, &val) in nums.iter().enumerate() {
            let norm = (((val as i128) % MOD) + MOD) % MOD;
            h[i + 1] = (h[i] * BASE + norm) % MOD;
            p[i + 1] = (p[i] * BASE) % MOD;
        }

        let mut lo = 1usize;
        let mut hi = n;

        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if Self::check(mid, n, &h, &p, MOD) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }

        lo as i32
    }

    fn check(len: usize, n: usize, h: &[i128], p: &[i128], mod_val: i128) -> bool {
        let mut cnt: HashMap<i128, usize> = HashMap::new();

        for l in 0..=(n - len) {
            let r = l + len;
            let key = (h[r] - ((h[l] * p[len]) % mod_val) + mod_val) % mod_val;
            *cnt.entry(key).or_insert(0) += 1;
        }

        cnt.values().any(|&times| times == 1)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {

        let nums: Vec<i32> = vec![2, 1, 2, 3, 3];
        assert_eq!(1, Solution::smallest_unique_subarray(nums));
    }
}
