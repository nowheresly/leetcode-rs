use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn distance(nums: Vec<i32>) -> Vec<i64> {
        let n = nums.len();
        let mut result = vec![0i64; n];
        let mut map: HashMap<i32, Vec<usize>> = HashMap::new();
        for (i, &val) in nums.iter().enumerate() {
            map.entry(val).or_insert(Vec::new()).push(i);
        }

        for (_, indices) in map {
            let size = indices.len();
            if size <= 1 {
                continue;
            }

            // Calculate total sum of indices for this value
            let total_sum: i64 = indices.iter().map(|&idx| idx as i64).sum();

            let mut prefix_sum: i64 = 0;

            for (i, &current_idx) in indices.iter().enumerate() {
                let current_idx_i64 = current_idx as i64;
                let left_count = i as i64;
                let right_count = (size - 1 - i) as i64;

                // suffix_sum = total_sum - prefix_sum - current_idx
                let suffix_sum = total_sum - prefix_sum - current_idx_i64;

                // Left side: (count * current_val) - sum_of_indices_to_left
                let left_dist = (left_count * current_idx_i64) - prefix_sum;

                // Right side: sum_of_indices_to_right - (count * current_val)
                let right_dist = suffix_sum - (right_count * current_idx_i64);

                result[current_idx] = left_dist + right_dist;

                // Update prefix sum for the next iteration
                prefix_sum += current_idx_i64;
            }
        }

        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![5,0,3,4,0], Solution::distance(vec![1,3,1,1,2]));
    }
}