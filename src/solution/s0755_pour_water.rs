pub struct Solution {}

impl Solution {
    pub fn pour_water(heights: Vec<i32>, volume: i32, k: i32) -> Vec<i32> {
        let mut res = heights.clone();
        let n = res.len();
        let k = k as usize;
        let mut volume = volume;

        while volume > 0 {
            volume -= 1;

            // search left if drop can fall
            let mut min_left_index = k;
            let mut min = res[k];
            for i in (0..k).rev() {
                if res[i] > min {
                    break;
                }
                if res[i] < min {
                    min_left_index = i;
                    min = res[i];
                }
            }

            // can fall left?
            if min_left_index != k {
                res[min_left_index] += 1;
                continue;
            }

            // cannot fall left
            // search right
            let mut min_right_index = k;
            min = res[k];
            for i in k+1..n {
                if res[i] > min {
                    break;
                }
                if res[i] < min {
                    min_right_index = i;
                    min = res[i];
                }
            }

            // can fall right?
            if min_right_index != k {
                res[min_right_index] += 1;
                continue;
            }

            // cannot fall right
            // inplace
            res[k] += 1;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![4,4,4,4,3,3,3,3,3,4,3,2,1], Solution::pour_water(vec![1,2,3,4,3,2,1,2,3,4,3,2,1], 10, 2));
    }
}
