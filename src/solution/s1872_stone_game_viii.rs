pub struct Solution {}

impl Solution {
    pub fn stone_game_viii(stones: Vec<i32>) -> i32 {
        let n = stones.len();
        let mut prefix = vec![0;n+1];
        for i in 0..n {
            prefix[i+1] = prefix[i] + stones[i];
        }
        let mut current_dp = prefix[n];
        for i in (1..(n-1)).rev() {
            current_dp = current_dp.max(prefix[i+1] - current_dp);
        }
        current_dp
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(5,  Solution::stone_game_viii(vec![-1,2,-3,4,-5]));
    }
}
