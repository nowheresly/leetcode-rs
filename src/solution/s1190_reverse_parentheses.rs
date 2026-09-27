
pub struct Solution {}

impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let bytes = s.as_bytes();
        let n = bytes.len();

        let mut pair = vec![0; n];
        let mut stack = Vec::new();

        // precompute matching parentheses indices
        for i in 0..n {
            if bytes[i] == b'(' {
                stack.push(i);
            } else if bytes[i] == b')' {
                if let Some(j) = stack.pop() {
                    pair[i] = j;
                    pair[j] = i;
                }
            }
        }

        let mut result = String::with_capacity(n);

        let mut curr : isize = 0;
        let mut direction: isize = 1;

        while curr >= 0 && curr < (n as isize) {
            let idx = curr as usize;
            let c = bytes[idx];

            if c == b'(' || c == b')' {
                curr = pair[idx] as isize;
                direction = -direction;
            } else {
                result.push(c as char);
            }

            curr += direction;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(String::from("leetcode"), Solution::reverse_parentheses(String::from("(ed(et(oc))el)")));

    }
}
