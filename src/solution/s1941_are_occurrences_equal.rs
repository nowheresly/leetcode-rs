pub struct Solution {}

impl Solution {
    pub fn are_occurrences_equal(s: String) -> bool {
        let mut freq = vec![0; 26];
        for c in s.chars() {
            freq[c as usize - 'a' as usize] += 1;
        }
        let mut same = -1;
        for i in 0..freq.len() {
            if freq[i] == 0 {
                continue;
            }
            if same == -1 {
                same = freq[i];
            } else if same != freq[i] {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            true,
            Solution::are_occurrences_equal(String::from("abacbc"))
        );
    }
}
