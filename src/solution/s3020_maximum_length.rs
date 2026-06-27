use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i64, i32> = HashMap::new();

        for &num in nums.iter() {
            *map.entry(num as i64).or_insert(0) += 1;
        }

        let mut res = 1;
        for &key in map.keys() {
            if key == 1 {
                let freq = map.get(&key).unwrap();
                res = res.max(if freq % 2 == 0 { *freq - 1 } else { *freq });
                continue;
            }

            let sqrt = (key as f64).sqrt();
            if sqrt == sqrt.trunc() {
                let sqrt_int = sqrt as i64;

                if map.get(&sqrt_int).copied().unwrap_or(0) >= 2 {
                    continue;
                }
            }

            let mut count = 0;
            let mut curr = key;

            while let Some(&freq) = map.get(&curr) {
                if freq >= 2 {
                    count += 1;
                    curr *= curr;

                    if curr > 1_000_000_000 {
                        break;
                    }
                } else {
                    count += 1;
                    break;
                }
            }

            res = res.max((count * 2) - 1);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::maximum_length(vec![5, 4, 1, 2, 2]));
    }
}
