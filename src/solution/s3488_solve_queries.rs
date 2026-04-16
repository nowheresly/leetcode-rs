use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn solve_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let mut res = vec![-1; queries.len()];
        let mut map:HashMap<i32, Vec<usize>> = HashMap::new();
        for i in 0..nums.len() {
            let val = nums[i];
            map.entry(val).or_insert(vec![]).push(i);
        }
        for i in 0..queries.len() {
            let q_idx = queries[i] as usize;
            let val = nums[q_idx];
            let mut dist = i32::MAX;
            let q_idx_i32 = q_idx as i32;
            if let Some(list) = map.get(&val) {
                if list.len() == 1 {
                    continue;
                }
                let index = find_index(&list, &q_idx);
                let size = list.len();
                dist = dist.min(i32::abs(q_idx_i32 - list[(index + size - 1) % size] as i32));
                dist = dist.min(nums.len() as i32 - i32::abs(q_idx_i32 - list[(index + size - 1) % size] as i32));

                dist = dist.min(i32::abs(q_idx_i32 - list[(index + 1) % size] as i32));
                dist = dist.min(nums.len() as i32 - i32::abs(q_idx_i32 - list[(index + 1) % size] as i32));
                res[i] = dist;

            }
        }
        res
    }
}

fn find_index(list: &&Vec<usize>, target: &usize) -> usize {
    let mut low = 0;
    let mut high = list.len();
    while low < high {
        let mid = low + (high - low) / 2;
        if list[mid] == *target {
            return mid;
        }
        if list[mid] < *target {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    low
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![2, -1, 3],
            Solution::solve_queries(vec![1, 3, 1, 4, 1, 3, 2], vec![0, 3, 5])
        );
    }
}
