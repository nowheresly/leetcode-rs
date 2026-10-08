
pub struct Solution {}

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut count_open = 0;
        let mut res = String::new();
        for c in s.chars() {
            if c == '(' {
                count_open += 1;
            } else if c == ')' {
                count_open -= 1;
            }
            if count_open > 1 {
                res.push(c);
            } else if count_open == 1 && c == ')' {
                res.push(c);
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
        assert_eq!("()()()", Solution::remove_outer_parentheses(String::from("(()())(())")));
    }
}
