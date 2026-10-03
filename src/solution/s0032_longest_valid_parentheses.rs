pub struct Solution {}
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut left = 0;
        let mut right = 0;
        let mut max = 0;
        let ch = s.chars().collect::<Vec<char>>();
        let n = ch.len();
        
        for i in 0..n {
            if ch[i] == '(' {
                left += 1;
            } else  {
                right += 1;
            }
            
            if left == right {
                max = max.max(right * 2);
            } else if right > left {
                left = 0;
                right = 0;
            }
        }

        for i in (0..n).rev() {
            if ch[i] == '(' {
                left += 1;
            } else  {
                right += 1;
            }

            if left == right {
                max = max.max(right * 2);
            } else if left > right {
                left = 0;
                right = 0;
            }
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::longest_valid_parentheses(String::from(")()())")));
    }
}
