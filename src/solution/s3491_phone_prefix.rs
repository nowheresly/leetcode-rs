pub struct Solution {}

impl Solution {
    pub fn phone_prefix(numbers: Vec<String>) -> bool {
        let mut numbers = numbers;
        numbers.sort();
        for i in 0..numbers.len() - 1 {
            if numbers[i + 1].starts_with(&numbers[i]) {
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
            Solution::phone_prefix(vec![
                String::from("1"),
                String::from("2"),
                String::from("4"),
                String::from("3")
            ])
        );
    }
}
