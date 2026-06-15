pub struct Solution {}

impl Solution {
    pub fn number_of_ways(num_people: i32) -> i32 {
        let modus = 1_000_000_007;
        let mut dp = vec![0i64; num_people as usize+1];

        dp[0] = 1;

        for i in (2..=num_people as usize).step_by(2) {

            for j in (2..=i).step_by(2) {

                let left_group = dp[j - 2];
                let right_group = dp[i - j];

                let total_ways = (left_group * right_group) % modus;

                dp[i] = (dp[i] + total_ways) % modus;
            }
        }

        dp[num_people as usize] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(5, Solution::number_of_ways(6));
    }
}
