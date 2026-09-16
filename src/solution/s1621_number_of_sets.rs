pub struct Solution {}
impl Solution {
    pub fn number_of_sets(n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let modu = 1_000_000_007;
        let mut dp0 = vec![vec![0; k as usize + 1]; n];
        let mut dp1 = vec![vec![0; k as usize + 1]; n];

        dp0[0][0] = 1;

        for i in 1..n {
            for j in 0..=k {
                dp0[i][j] = (dp0[i - 1][j] + dp1[i - 1][j]) % modu;

                dp1[i][j] = dp1[i - 1][j];

                if j > 0 {
                    let way_to_start = (dp0[i - 1][j - 1] + dp1[i - 1][j - 1]) % modu;
                    dp1[i][j] = (dp1[i][j] + way_to_start) % modu;
                }
            }
        }
        (dp0[n - 1][k] + dp1[n - 1][k]) % modu
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(5, Solution::number_of_sets(4, 2));
    }
}
