pub struct Solution {}

impl Solution {
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];
        for val in nums.iter() {
            let mut tmp = vec![];
            let mut val = *val;
            while val > 0 {
                tmp.push(val % 10);
                val /= 10;
            }
            for item in tmp.iter().rev() {
                res.push(*item);
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
            vec![1,3,2,5,8,3,7,7],
            Solution::separate_digits(vec![13,25,83,77])
        );
    }
}
