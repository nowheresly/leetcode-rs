pub struct Solution {}

impl Solution {
    pub fn added_integer(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut min1 = nums1[0];
        for &i in nums1.iter() {
            min1 = min1.min(i);
        }
        let mut min2 = nums2[0];
        for &i in nums2.iter() {
            min2 = min2.min(i);
        }
        min2 - min1
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::added_integer(vec![2,6,4], vec![9,7,5]));
    }
}