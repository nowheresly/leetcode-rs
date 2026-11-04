pub struct Solution {}

impl Solution {
    pub fn number_game(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut nums = nums;
        nums.sort();

        let mut ans = vec![0;n];
        let mut index = 0;
        for i in (0..n).step_by(2) {
            let val_alice = nums[i];
            let val_bob= nums[i + 1];
            ans[index] = val_bob;
            index +=1;
            ans[index] = val_alice;
            index +=1;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![3,2,5,4], Solution::number_game(vec![5,4,2,3]));
    }
}
