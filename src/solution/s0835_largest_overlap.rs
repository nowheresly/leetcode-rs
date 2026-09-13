use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        let n = img1.len();
        let mut ones1 = vec![];
        let mut ones2 = vec![];
        for i in 0..n {
            for j in 0..n {
                if img1[i][j] == 1 {
                    ones1.push((i, j));
                }
                if img2[i][j] == 1 {
                    ones2.push((i, j));
                }
            }
        }
        let mut max_overlap = 0;
        let mut map = HashMap::new();
        for &p1 in ones1.iter() {
            for &p2 in ones2.iter() {
                let dx = p2.0 as i32 - p1.0 as i32;
                let dy = p2.1 as i32 - p1.1 as i32;
                let key = (dx + 100) * 1000 + (dy + 100);

                map.entry(key).and_modify(|x| *x += 1).or_insert(1);
                max_overlap = max_overlap.max(map[&key]);
            }
        }
        max_overlap
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            3,
            Solution::largest_overlap(
                vec![vec![1, 1, 0], vec![0, 1, 0], vec![0, 1, 0]],
                vec![vec![0, 0, 0], vec![0, 1, 1], vec![0, 0, 1]]
            )
        );
    }
}
