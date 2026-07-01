pub struct Solution {}

impl Solution {
    pub fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
        let mut prices = prices;
        prices.sort();
        if prices.len() < 2 {
            return money;
        }
        let total = prices[0] + prices[1];
        if total > money {
            return money;
        }
        money - total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(0, Solution::buy_choco(vec![1,2,2], 3));
    }
}
