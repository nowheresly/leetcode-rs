pub struct Solution {}

impl Solution {
    pub fn total_waviness(num1: i32, num2: i32) -> i32 {
        let mut res = 0;
        for i in num1..=num2 {
            if i < 100 {
                continue;
            }
            let mut val = i;
            let mut prev = val % 10;
            val /= 10;
            let mut cur = val % 10;
            val /= 10;
            while val > 0 {
                let next = val % 10;
                val /= 10;
                if cur > prev && cur > next {
                    res += 1;
                } else if cur < prev && cur < next {
                    res += 1;
                }
                prev = cur;
                cur = next;
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
        assert_eq!(3, Solution::total_waviness(120, 130));

    }
}
