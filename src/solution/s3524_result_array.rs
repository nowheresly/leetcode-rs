pub struct Solution {}

impl Solution {
    pub fn result_array(nums: Vec<i32>, k: i32) -> Vec<i64> {
        let k = k as usize;
        let n = nums.len();
        let mut dp = vec![vec![0; k]; n];

        dp[0][nums[0] as usize % k] = 1;

        for i in 1..n {
            let remainder = nums[i] as usize % k;
            dp[i][remainder] += 1;

            for j in 0..k {
                let new_rem = (j * remainder) % k;
                dp[i][new_rem] += dp[i - 1][j];
            }
        }
        let mut res = vec![0; k];
        for i in 0..n {
            for j in 0..k {
                res[j] += dp[i][j];
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
            vec![9, 2, 4],
            Solution::result_array(vec![1, 2, 3, 4, 5], 3)
        );
    }
}
