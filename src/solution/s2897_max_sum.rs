
pub struct Solution {}
impl Solution {
    pub fn max_sum(nums: Vec<i32>, k: i32) -> i32 {
        let modus = 1_000_000_007;
        let mut bit_counts = vec![0; 32];

        for num in nums {
            for i in 0..32 {
                if ((num >> i) & 1) == 1 {
                    bit_counts[i] += 1;
                }
            }
        }

        let mut res:i64 = 0;

        for _i in 0..k as usize {
            let mut current_num:i64 = 0;

            for j in 0..32 {
                if bit_counts[j] > 0 {
                    current_num |= 1 << j;
                    bit_counts[j] -= 1;
                }
            }

            let square = (current_num * current_num) % modus;
            res = (res + square) % modus;
        }

        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(261, Solution::max_sum(vec![2,6,5,8], 2));

    }
}
