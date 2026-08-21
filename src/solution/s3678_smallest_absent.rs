use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn smallest_absent(nums: Vec<i32>) -> i32 {
        let mut total = 0;
        let mut set = HashSet::new();

        for i in nums.iter() {
            total += i;
            set.insert(i);
        }

        let start = if total >= 0 {
            (total / nums.len() as i32) + 1
        } else {
            1
        };

        for i in start..i32::MAX {
            if set.contains(&i) == false {
                return i;
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
        assert_eq!(2, Solution::smallest_absent(vec![4, -1]));
    }
}
