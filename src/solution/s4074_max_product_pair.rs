pub struct Solution {}

impl Solution {
    pub fn max_product_pair(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut res = vec![-1;2];
        let n = nums.len();
        let mut max = i32::MIN;
        for i in 0..n {
            for j in i+1..n {
                if nums[i] == nums[j] || nums[i] + nums[j] != target {
                    continue;
                }
                if nums[i] * nums[j] > max {
                    max = nums[i] * nums[j];
                    if nums[i] > nums[j] {
                        res[0] = i as i32;
                        res[1] = j as i32;
                    } else {
                        res[0] = j as i32;
                        res[1] = i as i32;
                    }
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
        assert_eq!(vec![2,1], Solution::max_product_pair(vec![1,2,3,4], 5));
    }
}
