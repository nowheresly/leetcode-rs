pub struct Solution {}

impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut ind1 = 0;
        let mut ind2 = 0;
        
        loop {
            if ind1 >= nums1.len() {
                break;
            }
            if ind2 >= nums2.len() {
                break;
            }
            let val1 = nums1[ind1];
            let val2 = nums2[ind2];
            if val1 == val2 {
                return val1;
            }
            if val1 < val2 {
                ind1 += 1;
                continue;
            }
            if val1 > val2 {
                ind2 += 1;
                continue;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::get_common(vec![1,2,3,6], vec![2,3,4,5]));

    }
}
