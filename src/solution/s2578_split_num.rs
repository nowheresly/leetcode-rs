pub struct Solution {}

impl Solution {
    pub fn split_num(num: i32) -> i32 {
        let mut freq = vec![0;10];
        let mut count = 0;
        let mut s = num;
        while s > 0 {
            let val = s % 10;
            s /= 10;
            freq[val as usize] += 1;
            count += 1;
        }
        let mut num1 = 0;
        let mut num2 = 0;
        let mut i = 0;
        let mut first = true;
        while count > 0 {
            count -= 1;
            while freq[i] == 0 {
                i += 1;
            }
            if first {
                num1 *= 10;
                num1 += i as i32;
            } else {
                num2 *= 10;
                num2 += i as i32;
            }
            first = !first;
            freq[i] -= 1;
        }
        num1 + num2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(59, Solution::split_num(4325));


    }
}
