pub struct Solution {}

impl Solution {
    pub fn largest_integer(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut count = vec![0; 51];
        for &i in nums.iter() {
            count[i as usize] += 1;
        }
        if k == 1 {
            for i in (0..51).rev() {
                if count[i] == 1 {
                    return i as i32;
                }
            }
            return -1;
        }
        if k == n as i32 {
            for i in (0..51).rev() {
                if count[i] != 0 && count[i] <= n {
                    return i as i32;
                }

            }
            return -1;
        }
        let mut res = -1;
        if count[nums[0] as usize] == 1 {
            res =nums[0];
        }
        if count[nums[n-1] as usize] == 1 {
            res = res.max(nums[n-1]);
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(7, Solution::largest_integer(vec![3,9,2,1,7], 3));
    }
}
