
pub struct Solution {}

impl Solution {
    pub fn account_balance_after_purchase(purchase_amount: i32) -> i32 {
        let unit = purchase_amount % 10;
        let tenth = purchase_amount / 10;
        if unit < 5 {
            return 100 - tenth * 10;
        }
        100 - (tenth * 10 + 10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(90, Solution::account_balance_after_purchase(9));
    }
}
