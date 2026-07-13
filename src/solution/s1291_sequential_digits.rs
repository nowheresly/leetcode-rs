
pub struct Solution {}

impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let mut res = vec![];
        let size_low = size_of(low);
        let size_high = size_of(high);

        for size in size_low..=size_high {
            for start in 1..(10 - size +1) {
                let mut val = start;
                let mut count_digits = 1;
                while count_digits < size {
                    val *= 10;
                    val += start + count_digits;
                    count_digits += 1;
                }
                if val < low  {
                    continue;
                }
                if val > high {
                    break;
                }
                res.push(val);
            }
        }
        res
    }
}

fn size_of(n: i32) -> i32 {
    let mut n = n;
    let mut res = 0;
    while n > 0 {
        n /= 10;
        res += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {

        assert_eq!(vec![1234,2345,3456,4567,5678,6789,12345]
                   , Solution::sequential_digits(1000, 13000));

    }
}
