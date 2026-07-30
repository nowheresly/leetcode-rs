pub struct Solution {}

impl Solution {
    pub fn count_complete_day_pairs(hours: Vec<i32>) -> i32 {
        let mut res = 0;
        let n = hours.len();
        for i in 0..n {
            for j in i + 1..n {
                if (hours[i] + hours[j]) % 24 == 0 {
                    res += 1;
                }
            }
        }
        res
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::count_complete_day_pairs(vec![12,12,30,24,24]));
    }
}