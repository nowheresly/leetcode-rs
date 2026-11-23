pub struct Solution {}
impl Solution {
    pub fn count_consistent(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut res = n as i32;
        for i in 0..n {
            let mut min = nums[i];
            let mut anded = nums[i];
            for j in (i + 1)..n {
                min = min.min(nums[j]);
                anded = anded & nums[j];
                if min == anded {
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

        let nums: Vec<i32> = vec![5, 3, 1, 2];
        assert_eq!(6, Solution::count_consistent(nums));
    }
}
