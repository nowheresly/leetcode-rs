

pub struct Solution {}

impl Solution {
    pub fn count_valid_prefixes(s: String) -> i32 {
        let mut res = 0;
        let mut count1= 0;
        let mut total = 0;
        for c in s.chars() {
            count1 += if c=='1' { 1 } else { 0 };
            total += 1;
            if i32::abs(2*count1 - total) <= 1 {
                res += 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            3,
            Solution::count_valid_prefixes(String::from("00101"))
        );
    }

}
