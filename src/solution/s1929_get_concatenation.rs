pub struct Solution {}
impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut res = vec![];
        for i in 0..n {
            res.push(nums[i]);
        }
        for j in 0..n {
            res.push(nums[j]);
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
            vec![1, 3, 2, 1, 1, 3, 2, 1],
            Solution::get_concatenation(vec![1, 3, 2, 1])
        );
    }
}
