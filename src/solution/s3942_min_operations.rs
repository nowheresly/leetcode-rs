pub struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return 0;
        }

        let mut start = 0;
        while nums[start] != 0 {
            start += 1;
        }

        let mut asc = true;
        let mut desc = true;

        for i in 1..n {
            let value = nums[(start + i) % n];

            if value != i as i32 {
                asc = false;
            }
            if value != n as i32 - i as i32 {
                desc = false;
            }

            if asc == false && desc == false {
                return -1;
            }
        }

        if asc {
            return (start as i32).min(n as i32 - start as i32 + 2);
        }
        ((n - start) as i32).min(start as i32 + 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(-1, Solution::min_operations(vec![2, 0, 1, 3]));
    }
}
