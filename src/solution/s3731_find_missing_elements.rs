use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn find_missing_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut min = i32::MAX;
        let mut max = 0;

        let mut set = HashSet::new();
        for i in nums {
            max = max.max(i);
            min = min.min(i);
            set.insert(i);
        }

        let mut res = vec![];
        for i in (min+1)..max {
            if set.contains(&i) == false {
                res.push(i);
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
        assert_eq!(vec![3], Solution::find_missing_elements(vec![1,4,2,5]));
    }
}
