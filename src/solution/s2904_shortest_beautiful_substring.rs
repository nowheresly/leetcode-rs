pub struct Solution {}

impl Solution {
    pub fn shortest_beautiful_substring(s: String, k: i32) -> String {
        let ch = s.chars().collect::<Vec<char>>();
        let n = ch.len();
        let mut l = 0;
        let mut count = 0;
        let mut res = String::from("");
        for r in 0..n {
            let c = ch[r];
            if c == '1' {
                count += 1;
            }
            while count > k {
                let d = ch[l];
                if d == '1' {
                    count -= 1;
                }
                l += 1;
            }
            while l <= r && ch[l] == '0' {
                l += 1;
            }
            if count == k {
                let str = s[l..=r].to_string();
                if res.len() == 0 {
                    res = str;
                    continue;
                }
                if res.len() > str.len() {
                    res = str;
                    continue;
                }
                if str.len() == res.len() && str < res {
                    res = str;
                }
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
        assert_eq!(String::from("1100100101011"), Solution::shortest_beautiful_substring(String::from("1100100101011001001"), 7));

    }
}
