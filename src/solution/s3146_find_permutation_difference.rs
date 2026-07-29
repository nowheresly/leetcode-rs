pub struct Solution {}

impl Solution {
    pub fn find_permutation_difference(s: String, t: String) -> i32 {
        let mut t_indices = [0_usize; 26];

        for (i, b) in t.bytes().enumerate() {
            t_indices[(b - b'a') as usize] = i;
        }

        let mut res = 0;

        for (i, b) in s.bytes().enumerate() {
            let t_idx = t_indices[(b - b'a') as usize];
            res += i.abs_diff(t_idx) as i32;
        }

        res
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(12, Solution::find_permutation_difference(String::from("abcde"), String::from("edbac")));
    }
}