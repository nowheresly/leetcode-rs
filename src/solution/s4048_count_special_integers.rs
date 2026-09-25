use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn count_special_integers(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut map:HashMap<i32, Vec<usize>> = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            map.entry(num).or_insert(vec![]).push(i);
        }
        for l in map.values() {
            if l.len() != 3 {
                continue;
            }
            if l[1] - l[0] == l[2] - l[1] {
                res += 1;
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
        assert_eq!(
            2,
            Solution::count_special_integers(vec![1, 8, 1, 5, 1, 5, 8, 5])
        );
    }
}
