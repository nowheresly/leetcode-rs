pub struct Solution {}

impl Solution {
    pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
        let mut freq = vec![0; 100_001];
        let mut max = 0;
        let mut res = -1;
        for i in nums.iter() {
            if *i & 1 == 0 {
                freq[*i as usize] += 1;
                if max < freq[*i as usize] {
                    max = freq[*i as usize];
                    res = *i;
                    continue;
                }
                if max == freq[*i as usize] {
                    if res > *i {
                        res = *i;
                    }
                }
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
            Solution::most_frequent_even(vec![0,1,2,2,4,4,1])
        );
    }
}
