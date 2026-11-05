
pub struct Solution {}

impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        let mut res = 0;
        for i in 0..arr1.len() {
            let val1 = arr1[i];
            let mut found = false;

            for j in 0..arr2.len() {
                let val2 = arr2[j];
                if i32::abs(val1 - val2) <= d {
                    found = true;
                    break;
                }
            }
            if found == false {
                res += 1;
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
        let ret = Solution::find_the_distance_value(vec![1,4,2,3], vec![-4,-3,6,10,20,30], 3);
        assert_eq!(2, ret);

    }
}
