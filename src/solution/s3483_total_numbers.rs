use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn total_numbers(digits: Vec<i32>) -> i32 {
        let mut set = HashSet::new();
        let n = digits.len();
        for i in 0..n {
            if digits[i] == 0 {
                continue;
            }
            for j in 0..n {
                if i == j {
                    continue;
                }
                for k in 0..n {
                    if j == k || i == k {
                        continue;
                    }
                    let val = digits[i] * 100 + digits[j] * 10 + digits[k];
                    if val % 2 == 1 {
                        continue;
                    }
                    set.insert(val);
                }
            }
        }
        set.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            2,
            Solution::total_numbers(vec![0,2,2])
        );
    }

}
