
pub struct Solution {}

impl Solution {
    pub fn furthest_distance_from_origin(moves: String) -> i32 {
        let n = moves.len() as i32;
        let mut count_r:i32 = 0;
        let mut count_l:i32 = 0;
        for c in moves.as_bytes() {
            if *c == b'R' {
                count_r += 1;
            } else  if *c == b'L' {
                count_l += 1;
            }
        }
        let count_under = n  - count_r - count_l;
        i32::max(count_l - count_r + count_under, count_r - count_l + count_under)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::furthest_distance_from_origin(String::from("L_RL__R")));

    }
}
