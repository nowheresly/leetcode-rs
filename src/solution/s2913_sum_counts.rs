use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn sum_counts(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut res = 0;
        let mut set = HashSet::new();
        for i in 1..=n {
            let size = i;
            for j in 0..(n-size+1) {
                for k in j..(j+size) {
                    set.insert(nums[k]);
                }
                res += set.len() * set.len();
                set.clear();
            }
        }
        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(15, Solution::sum_counts(vec![1,2,1]));

    }
}
