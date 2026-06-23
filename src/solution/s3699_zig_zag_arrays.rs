pub struct Solution {}

impl Solution {
    pub fn zig_zag_arrays(n: i32, l: i32, r: i32) -> i32 {
        let r = r as usize;
        let l = l as usize;
        let modus = 1_000_000_007;
        let mut dp0 = vec![0; r + 1];
        let mut dp1 = vec![0; r + 1];
        let mut sum0 = vec![0; r + 2];
        let mut sum1 = vec![0; r + 2];

        for i in l..=r {
            dp0[i] = 1;
            dp1[i] = 1;
            sum0[i] = i - l + 1;
            sum1[i] = i - l + 1;
        }

        for _ in 1..n {
            for j in l..=r {
                dp0[j] = (sum1[r] - sum1[j] + modus) % modus;
                dp1[j] = sum0[j - 1];
            }

            sum0[l] = dp0[l];
            sum1[l] = dp1[l];
            for j in l + 1..=r {
                sum0[j] = (sum0[j - 1] + dp0[j]) % modus;
                sum1[j] = (sum1[j - 1] + dp1[j]) % modus;
            }
        }
        ((sum0[r] + sum1[r]) % modus) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::zig_zag_arrays(3, 4, 5));
    }
}
