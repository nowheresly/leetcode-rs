pub struct Solution {}

impl Solution {
    pub fn maximum_element_after_decrementing_and_rearranging(arr: Vec<i32>) -> i32 {
        let mut arr = arr;
        arr.sort();
        let mut max_element = 1;
        for &num in &arr {
            if num >= max_element {
                max_element += 1;
            }
        }
        max_element - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::maximum_element_after_decrementing_and_rearranging(vec![2,2,1,2,1]));

    }
}
