pub struct Solution {}

impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let mut stack = vec![];
        stack.push(0);

        for c in s.chars() {
            if c == '(' {
                stack.push(0);
            } else {
                let v = stack.pop().unwrap();
                let w = stack.pop().unwrap();
                stack.push(w + 1.max(2*v));
            }
        }

        stack.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::score_of_parentheses(String::from("(())")));
    }
}
