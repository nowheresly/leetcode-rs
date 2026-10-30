pub struct Solution {}

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let n = citations.len();
        let mut left:i32 = 0;
        let mut right:i32 = n as i32 - 1;

        while left <= right {
            let mid = left + (right - left) / 2;

            let target = n as i32 - mid;
            if citations[mid as usize] == target {
                return citations[mid as usize];
            }

            if citations[mid as usize] < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        n as i32 - left
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::h_index(vec![0,1,3,5,6]));
        assert_eq!(1, Solution::h_index(vec![100]));
    }
}
