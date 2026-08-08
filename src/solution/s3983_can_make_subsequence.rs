pub struct Solution {}

impl Solution {
    pub fn can_make_subsequence(s: String, t: String) -> bool {
        let n = s.len();
        let m = t.len() as i32;
        if n > m as usize {
            return false;
        }
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut right_match:Vec<i32> = vec![0; n];
        let mut j = m - 1;
        // rightMatch[i] will store the maximum index in t where s.charAt(i) can be matched
        // such that the remaining characters of s also match correctly.
        for i in (0..n).rev() {
            while j >= 0 && t[j as usize] != s[i] {
                j -= 1;
            }
            right_match[i] = j;
            j -= 1;
        }
        let mut curr_t_idx = -1;
        for i in 0..n {
            // right_idx is where the suffix s[i+1...n-1] starts matching in t
            // If i is the last character, the suffix is empty, so we use m (length of t)
            let right_idx = if i + 1 < n { right_match[i + 1] } else { m };

            // If the suffix can be successfully matched (right_idx != -1)
            // And there is at least a 1 character gap between prefix end and suffix start
            if right_idx != -1 && curr_t_idx + 1 < right_idx {
                return true;
            }

            // Advance the prefix end for the next iteration
            curr_t_idx += 1;
            while curr_t_idx < m && t[curr_t_idx as usize] != s[i] {
                curr_t_idx += 1;
            }
            // If we run out of characters in t to match the current prefix, we can stop
            if curr_t_idx == m {
                break;
            }
        }
        false
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            true,
            Solution::can_make_subsequence(String::from("cat"), String::from("chat"))
        );
    }

}
