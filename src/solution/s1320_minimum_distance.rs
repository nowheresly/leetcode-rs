pub struct Solution {}

impl Solution {
    pub fn minimum_distance(word: String) -> i32 {
        // save[c] stores the max distance saved if one finger is on the PREVIOUS
        // character typed, and the other finger is resting on character 'c'.
        // Index 26 represents an "unplaced" finger.
        let mut save = vec![0; 27];
        let mut total_distance = 0;
        let n = word.len();
        let w = word.as_bytes();

        for i in 0..n - 1 {
            let prev = (w[i] - 'A' as u8) as i32;
            let cur = (w[i + 1] - 'A' as u8) as i32;

            let mut max_save_for_prev = 0;
            for j in 0..27 {
                max_save_for_prev = max_save_for_prev.max(save[j] + dist(prev, cur) - dist(j as i32, cur));
            }

            save[prev as usize] = max_save_for_prev;

            total_distance += dist(prev, cur);
        }

        let mut max_save = 0;
        for i in 0..27 {
            max_save = max_save.max(save[i]);
        }

        total_distance - max_save
    }
}

fn dist(a: i32, b: i32) -> i32 {
    if a == 26 || b == 26 {
        return 0;
    }
    let x1 = a / 6;
    let y1 = a % 6;
    let x2 = b / 6;
    let y2 = b % 6;
    return i32::abs(x1 - x2) + i32::abs(y1 - y2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::minimum_distance(String::from("CAKE")));
    }
}
