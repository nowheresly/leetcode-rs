pub struct Solution {}

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let mut res = 0;
        let mut stack = vec![];
        let ch: Vec<char> = s.chars().collect();
        let n = ch.len();
        let mut i = 0;
        while i < n {
            let c = ch[i];
            let next = if i < n - 1 { ch[i + 1] } else { '-' };
            if c == '(' {
                stack.push(1);
                i += 1;
                continue;
            }
            if c == ')' && next == ')' {
                i += 1;
            } else {
                res += 1;
            }
            if stack.is_empty() {
                res += 1;
            } else {
                stack.pop();
            }
            i += 1;
        }
        res += stack.len() as i32 * 2;
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::min_insertions(String::from("))())(")));
    }
}
