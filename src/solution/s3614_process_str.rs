pub struct Solution {}

impl Solution {
    pub fn process_str(s: String, k: i64) -> char {
        let n = s.len();
        let mut k = k;
        let mut len = vec![0i64;n];
        let mut current_len = 0i64;

        // Step 1: Calculate the virtual length of the string at each step
        let mut i = 0;
        for c in s.chars() {
            if c >= 'a' && c <= 'z' {
                current_len += 1;
            } else if c == '*' {
                // Length cannot drop below 0
                current_len = (current_len - 1).max(0);
            } else if c == '#' {
                // Duplicates the current length
                current_len *= 2;
            } else if c == '%' {
                // Reversing doesn't change the length
            }
            len[i] = current_len;
            i += 1;
        }

        // Step 2: Handle the out of bounds edge case immediately
        if n == 0 || k >= len[n - 1] || k < 0 {
            return '.';
        }

        // Step 3: Work backwards to trace where the k-th character originated
        let mut i = n - 1;
        for c in s.chars().rev() {
            if c >= 'a' && c <= 'z' {
                // If 'k' points exactly to the character that was appended here
                if k == len[i] - 1 {
                    return c;
                }
            } else if c == '*' {
                // The character was deleted from the very end.
                // Since our 'k' is already within the bounds of the smaller string,
                // the index remains completely unaffected when moving to the larger one.
            } else if c == '#' {
                // Map 'k' back to the original, pre-duplicated half
                if i > 0 && len[i - 1] > 0 {
                    k %= len[i - 1];
                }
            } else if c == '%' {
                // Map 'k' to its physical index before the reverse happened
                if i > 0 && len[i - 1] > 0 {
                    k = len[i - 1] - 1 - k;
                }
            }
            i -= 1;
        }

        '.'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            'd',
            Solution::process_str(
                String::from("cd%#*#"), 3
            )
        );
    }
}
