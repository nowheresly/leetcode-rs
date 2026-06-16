pub struct Solution {}

impl Solution {
    pub fn process_str(s: String) -> String {
        let mut result = String::new();

        for c in s.chars() {
            match c {
                'a'..='z' => result.push(c),

                '*' => { result.pop(); },

                '#' => result = result.repeat(2),

                '%' => result = result.chars().rev().collect(),

                _ => {}
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            String::from("ba"),
            Solution::process_str(
                String::from("a#b%*")
            )
        );
    }
}
