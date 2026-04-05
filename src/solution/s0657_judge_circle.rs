
pub struct Solution {}

impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut up = 0;
        let mut left = 0;
        for c in moves.chars() {
            if c == 'U' {
                up +=1;
            } else if c =='D' {
                up -=1;
            } else if c =='L' {
                left +=1;
            }else if c =='R' {
                left -=1;
            }
        }
        up == 0 && left == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {

        assert_eq!(true, Solution::judge_circle(String::from("UD")));

    }
}
