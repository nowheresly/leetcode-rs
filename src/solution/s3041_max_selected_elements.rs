use std::collections::HashMap;
pub struct Solution {}


impl Solution {
    pub fn max_selected_elements(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut dp = HashMap::<i32, i32>::new();
        let mut ans = 0;
        for x in nums {
            let increased = dp.get(&(x + 1)).copied().unwrap_or(0)
                .max(dp.get(&x).copied().unwrap_or(0) + 1);
            dp.insert(x + 1, increased);
            let unchanged = dp.get(&x).copied().unwrap_or(0)
                .max(dp.get(&(x - 1)).copied().unwrap_or(0) + 1);
            dp.insert(x, unchanged);
            ans = ans.max(increased).max(unchanged);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {

        let nums: Vec<i32> = vec![2, 1, 5, 1, 1];
        assert_eq!(3, Solution::max_selected_elements(nums));
    }
}
