
pub struct Solution {}

impl Solution {
    pub fn rotated_digits(n: i32) -> i32 {
        let mut ans = 0;
        for i in 0..=n {
            if good(i, false) {
                ans += 1;
            }
        }
        ans
    }
}

fn good(n:i32, flag:bool) -> bool {
    if n == 0 {
        return flag;
    }
    let d = n % 10;
    if d == 3 || d == 4 || d == 7 {
        return false;
    }
    if d == 0 || d == 1 || d == 8 {
        return good(n/10, flag);
    }
    good(n/10, true)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::rotated_digits(10));


    }
}
