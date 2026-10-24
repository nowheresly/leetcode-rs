pub struct Solution {}

impl Solution {
    pub fn max_capacity(costs: Vec<i32>, capacity: Vec<i32>, budget: i32) -> i32 {
        let n = capacity.len();
        let mut res = 0;

        let mut list1 = vec![];
        for i in 0..n {
            if costs[i] >= budget {
                // machine too expensive
                continue;
            }
            list1.push((costs[i], capacity[i]));
            res = res.max(capacity[i]);
        }
        // sort by cost
        list1.sort();

        let m = list1.len();
        let mut prefix_max = vec![0; m];
        for i in 0..m {
            let curr = list1[i];
            let max_partner_cost = budget - 1 - curr.0;
            let best_j = find_right_most_valid_index(&list1, 0, i as i32 - 1, max_partner_cost);
            if best_j != -1 {
                res = res.max(curr.1 + prefix_max[best_j as usize]);
            }
            prefix_max[i] = if i == 0 {
                curr.1
            } else {
                prefix_max[i - 1].max(curr.1)
            };
        }
        res
    }
}

fn find_right_most_valid_index(list: &Vec<(i32, i32)>, beg: i32, end: i32, target: i32) -> i32 {
    let mut res = -1;
    let mut beg = beg;
    let mut end = end;
    while beg <= end {
        let mid = beg + (end - beg) / 2;
        if list[mid as usize].0 <= target {
            res = mid as i32;
            beg = mid + 1;
        } else {
            end = mid - 1;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            8,
            Solution::max_capacity(vec![4, 8, 5, 3], vec![1, 5, 2, 7], 8)
        );
    }
}
