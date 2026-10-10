pub struct Solution {}

impl Solution {
    pub fn min_sum_square_diff(nums1: Vec<i32>, nums2: Vec<i32>, k1: i32, k2: i32) -> i64 {
        let n = nums1.len();
        let mut freq = vec![0; 100_001];
        for i in 0..n {
            let diff = i32::abs(nums1[i] - nums2[i]) as usize;
            freq[diff] += 1;
        }
        let mut k: i64 = k1 as i64 + k2 as i64;
        for i in (1..freq.len()).rev() {
            if freq[i] == 0 {
                continue;
            }
            if freq[i] < k {
                k -= freq[i];
                freq[i - 1] += freq[i];
                freq[i] = 0;
            } else {
                freq[i - 1] += k;
                freq[i] -= k;
                break;
            }
        }
        let mut res = 0;
        for i in 1..freq.len() {
            res += freq[i] * (i as i64 * i as i64);
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
            579,
            Solution::min_sum_square_diff(vec![1, 2, 3, 4], vec![2, 10, 20, 19], 0, 0)
        );
    }
}
