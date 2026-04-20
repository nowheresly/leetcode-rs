
pub struct Solution {}

impl Solution {
    pub fn max_distance(colors: Vec<i32>) -> i32 {
        let n = colors.len();
        let mut max_dist = 0;
        for i in (0..n-1).rev() {
            if colors[i] != colors[0] {
                max_dist = i;
                break;
            }
        }

        for i in 0..n {
            if colors[i] != colors[n-1] {
                max_dist = max_dist.max(n-1-i);
            }
        }
        max_dist as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {

        assert_eq!(4, Solution::max_distance(vec![1,8,3,8,3]));

    }
}
