use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let mut invalid_open = 0;
        let mut invalid_close = 0;

        for c in s.chars() {
            if c == '(' {
                invalid_open += 1;
            } else if c == ')' {
                if invalid_open > 0 {
                    invalid_open -= 1;
                } else if invalid_open == 0 {
                    invalid_close += 1;
                }
            }
        }

        let mut result: HashSet<String> = HashSet::new();
        bt(
            &mut result,
            &s,
            0,
            &mut String::new(),
            0,
            invalid_open,
            invalid_close,
        );
        result.into_iter().collect::<Vec<String>>()
    }
}

fn bt(
    result: &mut HashSet<String>,
    s: &str,
    index: usize,
    cur: &mut String,
    open: i32,
    invalid_open: i32,
    invalid_close: i32,
) {
    if index == s.len() {
        if open > 0 {
            return;
        }
        result.insert(cur.clone());
        return;
    }
    let c = s.as_bytes()[index] as char;
    if c == '(' {
        // we take it
        cur.push(c);
        bt(
            result,
            s,
            index + 1,
            cur,
            open + 1,
            invalid_open,
            invalid_close,
        );
        cur.remove(cur.len() - 1);
        // we skip it
        if invalid_open > 0 {
            bt(
                result,
                s,
                index + 1,
                cur,
                open,
                invalid_open - 1,
                invalid_close,
            );
        }
        return;
    }
    if c == ')' {
        // we take it if we can
        if open > 0 {
            cur.push(c);
            bt(
                result,
                s,
                index + 1,
                cur,
                open - 1,
                invalid_open,
                invalid_close,
            );
            cur.remove(cur.len() - 1);
        }
        // we skip it
        if invalid_close > 0 {
            bt(
                result,
                s,
                index + 1,
                cur,
                open,
                invalid_open,
                invalid_close - 1,
            );
        }
        return;
    }
    cur.push(c);
    bt(result, s, index + 1, cur, open, invalid_open, invalid_close);
    cur.remove(cur.len() - 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![String::from("(a())()"), String::from("(a)()()")],
            Solution::remove_invalid_parentheses(String::from("(a)())()"))
        );
    }
}
