use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut prefixes = HashSet::new();

        for val in arr1.iter() {
            let mut val = *val;
            while val > 0 {
                prefixes.insert(val);
                val /= 10;
            }
        }
        let mut max_len:i32 = 0;

        for val in arr2.iter() {
            let mut val = *val;
            while val > 0 {
                if prefixes.contains(&val) {
                    let len = i32::ilog10(val) as i32 + 1;
                    max_len = max_len.max(len);
                    break;
                }
                val /= 10;
            }
        }
        max_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::longest_common_prefix(vec![1,10,100], vec![1000]));
    }
}
