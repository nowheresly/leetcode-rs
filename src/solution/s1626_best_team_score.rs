

pub struct Solution {
}


#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Player {
    age: i32,
    score: i32,
}
impl Solution {
    pub fn best_team_score(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
        let n = scores.len();

        // Pair up ages and scores into a Vec<Player>
        let mut players: Vec<Player> = ages
            .into_iter()
            .zip(scores.into_iter())
            .map(|(age, score)| Player { age, score })
            .collect();

        // Automatically sorts by `age` ascending, then `score` ascending
        players.sort_unstable();

        let mut dp = vec![0; n];
        let mut ret = 0;

        for i in 0..n {
            let pi = players[i];
            dp[i] = pi.score;
            for j in (0..i).rev() {
                let pj = players[j];
                if pj.score <= pi.score {
                    dp[i] = dp[i].max(dp[j] + pi.score);
                }
            }
            ret = ret.max(dp[i]);
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let scores: Vec<i32> = vec![1, 3, 5, 10, 15];
        let ages: Vec<i32> = vec![1, 2, 3, 4, 5];
        assert_eq!(34, Solution::best_team_score(scores, ages));

    }
}
