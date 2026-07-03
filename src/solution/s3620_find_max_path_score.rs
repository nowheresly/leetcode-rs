use std::cmp::Reverse;
use std::collections::HashMap;

pub struct Solution {}
impl Solution {
    pub fn find_max_path_score(edges: Vec<Vec<i32>>, online: Vec<bool>, k: i64) -> i32 {
        let n = online.len();
        let mut adj = HashMap::new();
        for ed in edges {
            let (s,d,w) = (ed[0], ed[1], ed[2] as i64);
            if online[s as usize] && online[d as usize] {
                adj.entry(s).or_insert(vec![]).push((d, w));
            }
        }
        let mut left:i64 = 0;
        let mut right = k;
        let mut res = -1;
        while left <= right {
            let mid:i64 = (left + right) / 2;
            if check(&adj, n, mid, k) {
                res = res.max(mid);
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        res as i32
    }
}

fn check(adj: &HashMap<i32, Vec<(i32, i64)>>, n: usize, cost: i64, k: i64) -> bool {
    let mut shortest = HashMap::new();
    let mut min_heap = std::collections::BinaryHeap::new();
    min_heap.push(Reverse((0, 0))); // (cost, node)
    while let Some(Reverse((w1, n1))) = min_heap.pop() {
        if shortest.contains_key(&n1) {
            continue;
        }
        shortest.insert(n1, w1);
        if let Some(neighbors) = adj.get(&n1) {
            for &(n2, w2) in neighbors {
                if w2 < cost {
                    continue;
                }
                if !shortest.contains_key(&n2) {
                    min_heap.push(Reverse((w1+w2, n2)));
                }
            }
        }
    }
    *shortest.entry((n-1) as i32).or_insert(i64::MAX) <= k
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            3,
            Solution::find_max_path_score(
                vec![vec![0,1,5],vec![1,3,10],vec![0,2,3],vec![2,3,4]], vec![true,true,true,true], 10
            )
        );
    }
}
