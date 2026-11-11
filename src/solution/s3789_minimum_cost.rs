pub struct Solution {}

impl Solution {
    pub fn minimum_cost(cost1: i32, cost2: i32, cost_both: i32, need1: i32, need2: i32) -> i64 {
        let c1 = cost1 as i64;
        let c2 = cost2 as i64;
        let cb = cost_both as i64;
        let n1 = need1 as i64;
        let n2 = need2 as i64;

        let common = n1.min(n2);

        let mut total_cost = common * cb.min(c1+c2);

        if n1 > n2 {
            let rem1 = n1 - n2;
            total_cost += rem1 * c1.min(cb);
        } else {
            let rem2 = n2 - n1;
            total_cost += rem2 * c2.min(cb);
        }
        total_cost
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::minimum_cost(3, 2, 1, 3, 2));
    }
}
