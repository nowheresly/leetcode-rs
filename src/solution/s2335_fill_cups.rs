
pub struct Solution {}

impl Solution {
    pub fn fill_cups(amount: Vec<i32>) -> i32 {

        let mut amount = amount;
        amount.sort();

        let sum = amount.iter().sum::<i32>();

        amount[2].max((sum + 1) / 2)

    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {

        assert_eq!(7, Solution::fill_cups(vec![5,4,4]));

    }
}
