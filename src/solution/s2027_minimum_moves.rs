
pub struct Solution {}

impl Solution {
    pub fn minimum_moves(s: String) -> i32 {
        let mut res = 0;
        let s = s.as_bytes();
        let n = s.len();
        let mut i = 0;
        while i < n {
            if s[i] == b'O' {
                i += 1;
                continue;
            }
            i += 3;
            res += 1;
        }
        res
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {

        assert_eq!(2, Solution::minimum_moves(String::from("XXOX")));

    }
}
