pub struct Solution {}

impl Solution {
    pub fn closest_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
        let n = words.len() as i32;
        let mut res = i32::MAX;

        for i in 0..n {
            if words[i as usize] == target {
                res = res.min(i32::abs(i - start_index));
                res = res.min(n - i32::abs(i - start_index));
            }
        }
        if res == i32::MAX {
            return -1;
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
            1,
            Solution::closest_target(
                vec![
                    String::from("hello"),
                    String::from("i"),
                    String::from("am"),
                    String::from("leetcode"),
                    String::from("hello")
                ],
                String::from("hello"),
                1
            )
        );
    }
}
