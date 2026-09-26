pub struct Solution {}

impl Solution {
    pub fn ip_to_cidr(ip: String, n: i32) -> Vec<String> {
        let mut cur = to_int(&ip);
        let mut n = n;
        let mut res = vec![];
        while n > 0 {
            let max_bits = i32::trailing_zeros(cur) as usize;
            let mut bit_val = 1;
            let mut count = 0;
            while bit_val < n && count < max_bits {
                bit_val <<= 1;
                count += 1;
            }
            if bit_val > n {
                bit_val >>= 1;
                count -= 1;
            }
            res.push(to_str(cur, 32 - count));
            cur += bit_val;
            n -= bit_val;
        }
        res
    }
}

fn to_str(cur:i32, range : usize) -> String {
    let word_size = 8;
    let mut res = String::new();
    for i in (0..4).rev() {
        let word = (cur >> (i * word_size)) & ((1 << word_size) - 1);
        res.push_str(&word.to_string());
        if i > 0 {
            res.push('.');
        }
    }
    res.push_str(&format!("/{}", range));
    res
}

fn to_int(str: &str) -> i32 {
    let mut res = 0;
    for (i, c) in str.split('.').rev().enumerate() {
        res += c.parse::<i32>().unwrap() << (i * 8);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![
                String::from("255.0.0.7/32"),
                String::from("255.0.0.8/29"),
                String::from("255.0.0.16/32")
            ],
            Solution::ip_to_cidr(String::from("255.0.0.7"), 10)
        );
    }
}
