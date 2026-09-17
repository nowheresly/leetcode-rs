pub struct Solution {}

impl Solution {
    pub fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
        let n = arr.len();
        let mut min_len = vec![i32::MAX; n + 1];

        let mut l = 0;
        let mut sum = 0;
        let mut ans = i32::MAX;

        for r in 1..=n {
            sum += arr[r - 1];

            while sum > target && l < r {
                sum -= arr[l];
                l += 1;
            }

            min_len[r] = min_len[r - 1];
            if sum == target {
                let len = r - l;
                if min_len[l] != i32::MAX {
                    ans = ans.min(min_len[l] + len as i32);
                }
                min_len[r] = min_len[r].min(len as i32);
            }
        }

        if ans == i32::MAX { -1 } else { ans }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            23,
            Solution::min_sum_of_lengths(
                vec![
                    2, 2, 4, 4, 4, 4, 4, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1
                ],
                20
            )
        );
    }
}
