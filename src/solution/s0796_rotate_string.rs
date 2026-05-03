pub struct Solution {}

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }
        let doubled = s.repeat(2);
        doubled.contains(&goal)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::rotate_string(String::from("abcde"), String::from("cdeab")));
    }
}
