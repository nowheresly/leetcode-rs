pub struct Solution {}

impl Solution {
    pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
        let n = nums.len() as i32;

        for d in 0..n {
            if start + d < n && nums[(start + d) as usize] == target {
                return d;
            }
            if start - d >= 0 && nums[(start - d) as usize] == target {
                return d;
            }
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::get_min_distance(vec![1,2,3,4,5], 5, 3));

    }
}
