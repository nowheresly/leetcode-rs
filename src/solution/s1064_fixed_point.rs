
pub struct Solution {}

impl Solution {
    pub fn fixed_point(arr: Vec<i32>) -> i32 {
        for i in 0..arr.len() {
            if arr[i] == i as i32 {
                return i as i32;
            }
        }
        -1
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(0, Solution::fixed_point(vec![0,2,5,8,17]));
    }
}
