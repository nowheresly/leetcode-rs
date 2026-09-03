pub struct Solution {}

impl Solution {
    pub fn uniform_array(nums1: Vec<i32>) -> bool {
        let mut nums1 = nums1;
        nums1.sort();
        if nums1[0] % 2 == 1 {
            return true;
        }
        for i in 1..nums1.len() {
            if nums1[i] % 2 == 1 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            true,
            Solution::uniform_array(vec![1,4,7])
        );
    }

}
