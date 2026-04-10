use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn minimum_distance(nums: Vec<i32>) -> i32 {
        let mut res = i32::MAX;
        let mut map: HashMap<i32, Vec<usize>> = HashMap::new();
        for i in 0..nums.len() {
            if map.contains_key(&nums[i]) {
                map.get_mut(&nums[i]).unwrap().push(i);
                if map.get(&nums[i]).unwrap().len() > 2 {
                    let vec = map.get(&nums[i]).unwrap();
                    res = res.min(
                        (vec[vec.len() - 2] - vec[vec.len() - 3] + vec[vec.len() - 1]
                            - vec[vec.len() - 2]
                            + vec[vec.len() - 1]
                            - vec[vec.len() - 3]) as i32,
                    );
                }
            } else {
                map.insert(nums[i], vec![i]);
            }
        }
        if res == i32::MAX {
            return -1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(8, Solution::minimum_distance(vec![1, 1, 2, 3, 2, 1, 2]));
    }
}
