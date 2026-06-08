

pub struct Solution {}

impl Solution {
    pub fn share_candies(candies: Vec<i32>, k: i32) -> i32 {
        let mut res = 0;
        let k = k as usize;
        let mut freq = vec![0; 100001];
        let mut count = 0;
        for num in candies.iter() {
            freq[*num as usize] += 1;
            if freq[*num as usize] == 1 {
                count += 1;
            }
        }
        for i in 0..k {
            freq[candies[i] as usize] -= 1;
            if freq[candies[i] as usize] == 0 {
                count -= 1;
            }
        }
        res = res.max(count);

        for i in k..candies.len() {
            freq[candies[i-k] as usize] += 1;
            if freq[candies[i-k] as usize] == 1 {
                count += 1;
            }
            freq[candies[i] as usize] -= 1;
            if freq[candies[i] as usize] == 0 {
                count -= 1;
            }
            res = res.max(count);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {

        assert_eq!(3, Solution::share_candies(vec! [1,2,2,3,4,3], 3));
    }
}
