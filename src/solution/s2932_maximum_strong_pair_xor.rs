pub struct Solution {}

impl Solution {
    pub fn maximum_strong_pair_xor(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut res = 0;
        for i in 0..n {
            let x = nums[i];
            for j in (i+1)..n {

                let y = nums[j];
                if i32::abs(x - y) <= i32::min(x , y) {
                    let xor = x ^y;
                    res = res.max(xor);
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
        assert_eq!(7, Solution::maximum_strong_pair_xor(vec![1,2,3,4,5] ));

    }
}
