
pub struct Solution {}

impl Solution {
    pub fn min_changes(n: i32, k: i32) -> i32 {
        if (n & k) != k {
            return -1;
        }
        (n ^ k).count_ones() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::min_changes(13, 4));
    }
}
