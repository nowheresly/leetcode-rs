pub struct Solution {}

impl Solution {
    pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut m = 0;
        for &i in nums.iter() {
            m = m.max(i);
        }
        let mut u = 1;
        while u <= m {
            u <<= 1;
        }
        let mut s = vec![false; u as usize];
        for i in 0..n {
            for j in i..n {
                s[(nums[i] ^ nums[j]) as usize] = true;
            }
        }
        let mut t = vec![false; u as usize];
        for x in 0..u as usize {
            if s[x] == false {
                continue;
            }
            for &v in nums.iter() {
                t[x ^ v as usize] = true;
            }
        }
        let mut res = 0;
        for b in t {
            if b {
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
        assert_eq!(4, Solution::unique_xor_triplets(vec![6,7,8,9]));
    }


}
