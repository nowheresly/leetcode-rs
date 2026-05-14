
pub struct Solution {}

impl Solution {
    pub fn is_good(nums: Vec<i32>) -> bool {
        let n = nums.len() - 1;
        if n < 1 {
            return false;
        }

        let mut freq = vec![0;n+1];
        for num in nums.iter() {
            if *num < 1 || *num > n as i32 {
                return false;
            }
            freq[*num as usize] += 1;
        }
        for i in 1..n {
            if freq[i] != 1 {
                return false;
            }
        }
        freq[n] == 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(false, Solution::is_good(vec![3, 4, 4, 1, 2, 1]));
    }
}
