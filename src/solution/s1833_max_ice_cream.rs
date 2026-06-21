pub struct Solution {}

impl Solution {
    pub fn max_ice_cream(costs: Vec<i32>, coins: i32) -> i32 {
        let mut coins = coins;
        let mut bucket: Vec<i32> = vec![0; 100_001];
        for &c in costs.iter() {
            bucket[c as usize] += 1;
        }
        let mut count: i32 = 0;
        for i in 0..bucket.len() {
            if bucket[i] == 0 {
                continue;
            }
            let val = bucket[i] * i as i32;
            if val <= coins {
                coins -= val;
                count += bucket[i];
                continue;
            }
            count += bucket[i].min(coins / i as i32);
            break;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::max_ice_cream(vec![1, 3, 2, 4, 1], 7));
        assert_eq!(4, Solution::max_ice_cream(vec![10,2,10,10,10,10,8,2,7,8], 25));
    }
}
