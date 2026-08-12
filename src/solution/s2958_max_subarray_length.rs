use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut res = 1;
        let n = nums.len();
        let mut l = 0;
        let mut map = HashMap::new();
        map.insert(nums[0], 1);
        for r in 1..n {
            let val = nums[r];
            map.entry(val).and_modify(|v| *v += 1).or_insert(1);
            if *map.get(&val).unwrap() <= k {
                res = res.max(r - l + 1);
                continue;
            }
            while *map.get(&val).unwrap() > k {
                map.entry(nums[l]).and_modify(|v| *v -= 1).or_insert(0);
                l += 1;
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
        assert_eq!(6, Solution::max_subarray_length(vec![1,2,3,1,2,3,1,2], 2));
    }
}
