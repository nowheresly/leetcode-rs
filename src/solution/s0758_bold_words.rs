pub struct Solution {}

impl Solution {
    pub fn bold_words(words: Vec<String>, s: String) -> String {
        let mut bold = vec![0; s.len() + 1];
        for word in &words {
            let mut start = 0;

            while let Some(pos) = s[start..].find(word) {
                let actual_pos = start + pos;
                bold[actual_pos] += 1;
                bold[actual_pos + word.len()] -= 1;
                start = actual_pos + 1;
            }
        }

        let mut res = String::with_capacity(s.len() + 20);
        let mut val = 0;
        for (i, c) in s.char_indices() {
            if val + bold[i] > 0 && val <= 0 {
                // getting bold
                res.push_str("<b>");
            } else if val + bold[i] <= 0 && val > 0 {
                // no more bold
                res.push_str("</b>");
            }
            val += bold[i];
            res.push(c);
        }
        if val + bold[s.len()] <= 0 && val > 0 {
            // no more bold
            res.push_str("</b>");
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            String::from("a<b>ab</b>cd"),
            Solution::bold_words(
                vec![String::from("ab"), String::from("cb")],
                String::from("aabcd")
            )
        );
    }
}
