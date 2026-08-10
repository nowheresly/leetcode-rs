
pub struct Solution {}

impl Solution {
    pub fn winner_square_game(n: i32) -> bool {
        let n = n as usize;
        let mut dp = vec![false; n+1];

        for i in 0..=n {
            for k in 1..=i {
                if k*k > i {
                    break;
                }

                if dp[i - k*k] == false {
                    dp[i] = true;
                    break;
                }
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
        assert_eq!(true, Solution::winner_square_game(4));

    }
}
