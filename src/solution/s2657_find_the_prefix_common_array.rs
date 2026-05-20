pub struct Solution {}

impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let n = a.len();
        let mut res = vec![0; n];

        let mut freq = vec![0; n+1];
        let mut common_count = 0;

        for i in 0..n {
            freq[a[i] as usize] += 1;
            if freq[a[i] as usize] == 2 {
                common_count += 1;
            }

            freq[b[i] as usize] += 1;
            if freq[b[i] as usize] == 2 {
                common_count += 1;
            }

            res[i] = common_count;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![0,2,3,4], Solution::find_the_prefix_common_array(vec![1,3,2,4], vec![3,1,2,4]));

    }
}
