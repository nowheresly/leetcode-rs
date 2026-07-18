
pub struct Solution {}

impl Solution {
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        let mut min = nums[0];
        let mut max = nums[0];
        for &i in nums.iter() {
            min = min.min(i);
            max = max.max(i);
        }
        gcd(max, min)
    }
}

fn gcd(a:i32, b:i32) -> i32 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {

        assert_eq!(1, Solution::find_gcd(vec![7,5,6,8,3]));


    }
}
