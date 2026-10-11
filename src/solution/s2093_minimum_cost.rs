use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution {}

impl Solution {
    pub fn minimum_cost(n: i32, highways: Vec<Vec<i32>>, discounts: i32) -> i32 {
        let n = n as usize;
        let mut adj = vec![vec![]; n];
        for h in highways {
            let u = h[0] as usize;
            let v = h[1] as usize;
            let toll = h[2];
            adj[u].push((v, toll));
            adj[v].push((u, toll));
        }

        let mut dist = vec![vec![i32::MAX; discounts as usize + 1]; n as usize];
        dist[0][discounts as usize] = 0;

        let mut pq = BinaryHeap::new();
        pq.push((Reverse(0i32), 0, discounts));

        while let Some(curr) = pq.pop() {
            let Reverse(cost) = curr.0;
            let city = curr.1;
            let disc = curr.2;

            if city == n - 1 {
                return cost;
            }

            if cost > dist[city][disc as usize] {
                continue;
            }

            for edge in adj[city].iter() {
                let next_city = edge.0;
                let toll = edge.1;

                // option 1 do not use discount
                if cost + toll < dist[next_city][disc as usize] {
                    dist[next_city][disc as usize] = cost + toll;
                    pq.push((Reverse(cost + toll), next_city, disc));
                }

                // option 2 use discount
                if disc > 0 && cost + toll / 2 < dist[next_city][disc as usize - 1] {
                    dist[next_city][disc as usize - 1] = cost + toll / 2;
                    pq.push((Reverse(cost + toll / 2), next_city, disc - 1));
                }
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            8,
            Solution::minimum_cost(
                4,
                vec![
                    vec![1, 3, 17],
                    vec![1, 2, 7],
                    vec![3, 2, 5],
                    vec![0, 1, 6],
                    vec![3, 0, 20]
                ],
                20
            )
        );
    }
}
