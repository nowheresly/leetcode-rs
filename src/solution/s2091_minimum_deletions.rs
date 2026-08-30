pub struct Solution {}

impl Solution {
    pub fn minimum_deletions(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        
        let mut max = i32::MIN;
        let mut min = i32::MAX;
        let mut pos_min = 0;
        let mut pos_max = 0;
        
        for i in 0..n {
            let val = nums[i];
            if val < min {
                min = val;
                pos_min = i;
            }
            if val > max {
                max = val;
                pos_max = i;
            }
        }
        let low_pos = pos_min.min(pos_max) as i32;
        let high_pos = pos_min.max(pos_max) as i32;
        // remove all from the beginning
        let mut res = high_pos + 1;
        // remove all from the end
        res = res.min(n as i32 - low_pos);
        // remove from both sides
        res = res.min(low_pos + n as i32 - high_pos + 1);
        
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::minimum_deletions(vec![0,-4,19,1,8,-2,-3,5]));
    }
}
