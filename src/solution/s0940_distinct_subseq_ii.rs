pub struct Solution {}

impl Solution {
    pub fn distinct_subseq_ii(s: String) -> i32 {
        let modus:i64 = 1_000_000_007;
        let mut last = vec![0; 26];

        let chars = s.chars().collect::<Vec<char>>();
        let mut res:i64 = 1;
        for c in chars {
            let index = (c as u8 - b'a') as usize;
            let prev = res;
            res = (res * 2 + modus) - last[index];
            res %= modus;
            last[index] = prev;
        }
        res = res - 1 + modus;
        (res % modus) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(7, Solution::distinct_subseq_ii(String::from("abc")));
    }
}
