
pub struct Solution {}

impl Solution {
    pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        let n = values.len();
        let mut besti = values[0];
        let mut res = 0;
        for j in 1..n {
            let valj = values[j] - j as i32;
            res = res.max(besti + valj);
            besti = besti.max(values[j] + j as i32);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(7, Solution::max_score_sightseeing_pair(vec![1,3,5]));
    }
}
