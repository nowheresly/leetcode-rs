pub struct Solution {}

impl Solution {
    pub fn most_visited(n: i32, rounds: Vec<i32>) -> Vec<i32> {
        let mut ret = vec![];
        let start = rounds[0];
        let end = rounds[rounds.len() - 1];
        if start <= end {
            for i in start..=end {
                ret.push(i);
            }
        } else {
            for i in 1..=end {
                ret.push(i);
            }
            for i in start..=n {
                ret.push(i);
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![1,2,3,4,5,6,7], Solution::most_visited(7, vec![1,3,5,7]));

    }
}
