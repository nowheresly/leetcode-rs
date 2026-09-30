pub struct Solution {}

impl Solution {
    pub fn max_depth_after_split(seq: String) -> Vec<i32> {
        let mut depth = 0;

        seq.chars()
            .map(|c| {
                if c == '(' {
                    let ans = depth % 2;
                    depth += 1;
                    ans
                } else {
                    depth -= 1;
                    depth % 2
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![0, 0, 0, 1, 1, 0, 0, 0],
            Solution::max_depth_after_split(String::from("()(())()"))
        );
    }
}
