pub struct Solution {}

impl Solution {
    pub fn max_distance(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = 0;
        let mut res = 0;

        while i < nums1.len() && j < nums2.len() {
            if nums1[i] <= nums2[j] {
                res = res.max(j as i32 - i as i32);
                j += 1;
            } else {
                i += 1;
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
        assert_eq!(2, Solution::max_distance(vec![55,30,5,4,2], vec![100,20,10,10,5]));
    }
}
