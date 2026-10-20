pub struct Solution {}

impl Solution {
    pub fn sortable_integers(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut fac = Vec::new();

        let mut x = 1;
        while x * x <= n {
            if n % x == 0 {
                fac.push(x);
                if n / x != x {
                    fac.push(n / x);
                }
            }
            x += 1;
        }

        let mut target = nums.clone();
        target.sort();

        // Check whether cyclic rotations of nums1 can produce nums2
        fn check(nums1: &[i32], nums2: &[i32]) -> bool {
            let l = nums1.len();

            for i in 0..l.saturating_sub(1) {
                if nums1[i] > nums1[i + 1] {
                    let mut rotated = Vec::with_capacity(l);
                    rotated.extend_from_slice(&nums1[i + 1..]);
                    rotated.extend_from_slice(&nums1[..i + 1]);
                    return rotated == nums2;
                }
            }

            nums1 == nums2
        }

        let mut ans = 0;

        for d in fac {
            let mut all_good = true;
            let mut left = 0;
            while left < n {
                let right = left + d - 1;

                if !check(&nums[left..=right], &target[left..=right]) {
                    all_good = false;
                    break;
                }
                left += d;
            }
            if all_good {
                ans += d as i32;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(0, Solution::sortable_integers(vec![7, 6, 5]));
    }
}
