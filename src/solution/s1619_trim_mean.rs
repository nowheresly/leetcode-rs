
pub struct Solution {}
impl Solution {
    pub fn trim_mean(arr: Vec<i32>) -> f64 {
        let mut arr = arr;
        arr.sort();
        let n = arr.len();
        let nb_skip = 5 * n / 100;
        let mut res:f64 = 0.0;

        for i in nb_skip..(n-nb_skip) {
            res += arr[i] as f64;
        }
        res / (n as f64 - 2.0 * nb_skip as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4.77778, (Solution::trim_mean(    vec![6,0,7,0,7,5,7,8,3,4,0,7,8,1,6,8,1,1,2,4,8,1,9,5,4,3,8,5,10,8,6,6,1,0,6,10,8,2,3,4]) * 100_000.0).round()/100_000.0);
    }
}
