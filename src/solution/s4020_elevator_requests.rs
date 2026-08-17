

pub struct Solution {}

impl Solution {
    pub fn elevator_requests(_n: i32, requests: Vec<i32>) -> i32 {
        let mut res = requests[0];

        for i in 1..requests.len() {
            let delta = i32::abs(requests[i] - requests[i-1]);
            res += delta;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            7,
            Solution::elevator_requests(5, vec![2,1,4,3])
        );
    }

}
