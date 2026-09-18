pub struct Solution {}

impl Solution {
    pub fn max_num_of_substrings(s: String) -> Vec<String> {
        let bytes = s.as_bytes();
        let n = bytes.len();

        let mut first = [usize::MAX; 26];
        let mut last = [0; 26];

        for i in 0..n {
            let idx = (bytes[i] - b'a') as usize;
            if first[idx] == usize::MAX {
                first[idx] = i;
            }
            last[idx] = i;
        }

        let mut valids = Vec::new();

        for i in 0..n {
            let c_idx = (bytes[i] - b'a') as usize;
            if i != first[c_idx] {
                continue;
            }

            let mut r = last[c_idx];
            let mut valid = true;
            let mut j = i;

            // While loop allows dynamically increasing `r` as new characters are discovered
            while j <= r {
                let curr_idx = (bytes[j] - b'a') as usize;
                if first[curr_idx] < i {
                    valid = false;
                    break;
                }
                r = r.max(last[curr_idx]);
                j += 1;
            }

            if valid {
                valids.push((i, r));
            }
        }

        valids.sort_by_key(|&(_, end)| end);

        let mut res = Vec::new();
        let mut prev_end: i32 = -1;

        for (start, end) in valids {
            if start as i32 > prev_end {
                res.push(s[start..=end].to_string());
                prev_end = end as i32;
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
        assert_eq!(
            vec!["e", "f", "ccc"],
            Solution::max_num_of_substrings(String::from("adefaddaccc"))
        );
    }
}
