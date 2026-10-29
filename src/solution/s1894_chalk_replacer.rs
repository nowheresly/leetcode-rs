pub struct Solution {}

impl Solution {
    pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
        let mut sum:i64 = 0;
        for i in 0..chalk.len() {
            sum += chalk[i] as i64;
            if sum > k as i64 {
                break;
            }
        }
        let mut k = (k as i64 % sum) as i32;
        for i in 0..chalk.len() {
            if k < chalk[i] {
                return i as i32;
            }
            k = k - chalk[i];
        }
        0
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            0,
            Solution::chalk_replacer(
                vec![5,1,5], 22
            )
        );
    }
}
