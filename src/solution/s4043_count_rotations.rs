

pub struct Solution {}

impl Solution {
    pub fn count_rotations(s: String, k: i32) -> i32 {
        let n = s.len();
        if n <= 1 {
            return if k == 0 { n as i32 } else { 0 };
        }

        let bytes = s.as_bytes();
        let mut score = 0;

        // Calculate score for the first window (rotation 0)
        for j in 0..n - 1 {
            if bytes[j] == bytes[j + 1] {
                score += 1;
            }
        }

        let mut res = 0;
        if score == k {
            res += 1;
        }

        // Slide the window across rotations 1..n in O(1) per step
        for i in 0..n - 1 {
            // Remove the adjacent pair leaving the window
            if bytes[i] == bytes[(i + 1) % n] {
                score -= 1;
            }
            // Add the new adjacent pair entering the window
            if bytes[(i + n - 1) % n] == bytes[(i + n) % n] {
                score += 1;
            }

            if score == k {
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
            1,
            Solution::count_rotations(String::from("abca"), 0)
        );
    }

}
