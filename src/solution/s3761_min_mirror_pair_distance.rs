use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn min_mirror_pair_distance(nums: Vec<i32>) -> i32 {
        let mut res = i32::MAX;
        let n = nums.len();
        let mut map = HashMap::new();
        for i in 0..n {
            if map.contains_key(&nums[i]) {
                let prev_index = map.get(&nums[i]).unwrap();
                res = res.min(i as i32 - prev_index);
            }
            let rev = reverse(nums[i]);
            map.insert(rev, i as i32);
        }
        if res == i32::MAX {
            return -1;
        }
        res
    }
}

fn reverse(val: i32) -> i32 {
    let mut res = 0;
    let mut val = val;
    while val > 0 {
        res = res * 10 + val % 10;
        val /= 10;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::min_mirror_pair_distance(vec![12,21,45,33,54]));
    }
}
