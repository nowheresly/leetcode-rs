
pub struct Solution {}

impl Solution {
    pub fn find_kth_smallest(coins: Vec<i32>, k: i32) -> i64 {
        let k = k as i64;
        let mut begin:i64 = 0;
        let mut end : i64 = k * 25;
        let mut res = 1;

        while begin <= end {
            let mid = (begin + end) / 2;
            if count(&coins, mid) >= k {
                res = mid;
                end = mid - 1;
            } else {
                begin = mid + 1;
            }
        }
        res
    }
}

fn count(coins: &Vec<i32>, x: i64) -> i64 {
    dfs(coins, x, 0, 0, 1)
}

fn dfs(coins: &Vec<i32>, x: i64, index:usize, selected: i32, current_lcm:i64) -> i64 {
    if index == coins.len() {
        if selected > 0 {
            if selected % 2 == 0 {
                return -(x / current_lcm);
            }
            return x / current_lcm;
        }
        return 0;
    }
    // skip
    let val1 = dfs(coins, x, index + 1, selected, current_lcm);
    let next_lcm = current_lcm * coins[index] as i64 / gcd(current_lcm, coins[index] as i64);
    let val2 = dfs(coins, x, index + 1, selected+1, next_lcm);
    val1 + val2
}

fn gcd(a: i64, b:i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(9, Solution::find_kth_smallest(vec![3,6,9], 3));
    }
}
