
pub struct Solution {}


impl Solution {
    pub fn minimum_total_distance(robot: Vec<i32>, factory: Vec<Vec<i32>>) -> i64 {
        let mut robot = robot;
        let mut factory = factory;

        // 1. Sort robots and factories by position
        robot.sort();
        factory.sort_by(|a, b| a[0].cmp(&b[0]));

        // 2. Flatten factories based on their limits
        let mut factory_list = vec![];
        for f in factory {
            for _i in 0..f[1] {
                factory_list.push(f[0]);
            }
        }

        let n = robot.len();
        let m = factory_list.len();

        // 3. DP table: dp[i] is min distance for first i robots
        // We use a 1D array to save space (processing factory by factory)
        let mut dp = vec![1_000_000_000_000_000i64; n + 1];
        dp[0] = 0;

        // 4. Fill DP
        // For each factory...
        for j in 0..m {
            // Iterate backwards through robots to use the 1D DP trick
            for i in (1..=n).rev() {
                let current_dist = i64::abs(robot[i-1] as i64 - factory_list[j] as i64);
                // Option: Use this factory for robot i, or keep previous best
                dp[i] = i64::min(dp[i], dp[i - 1] + current_dist);
            }
        }
        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::minimum_total_distance(vec![0,4,6], vec![vec![2,2],vec![6,2]]));

    }
}
