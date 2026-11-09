use std::collections::{BTreeSet, BinaryHeap};
use std::cmp::Reverse;

pub struct Solution {}

impl Solution {
    pub fn busiest_servers(k: i32, arrival: Vec<i32>, load: Vec<i32>) -> Vec<i32> {
        let k_usize = k as usize;

        // 1. Populate BTreeSet with available server IDs (0 to k - 1)
        let mut available: BTreeSet<usize> = (0..k_usize).collect();

        // 2. Min-Heap storing Reverse((finish_time, server_id))
        let mut busy: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();

        let mut count = vec![0; k_usize];
        let mut max_requests = 0;

        // 3. Process each request
        for (i, (&start, &duration)) in arrival.iter().zip(load.iter()).enumerate() {
            let start_time = start as i64;
            let finish_time = start_time + (duration as i64);

            // Wake up servers that have finished by start_time
            while let Some(&Reverse((end, server_id))) = busy.peek() {
                if end <= start_time {
                    busy.pop();
                    available.insert(server_id);
                } else {
                    break;
                }
            }

            // Drop the request if no servers are free
            if available.is_empty() {
                continue;
            }

            // Find the first available server >= (i % k), or wrap around
            let target = i % k_usize;
            let assigned = *available
                .range(target..)
                .next()
                .or_else(|| available.iter().next())
                .unwrap();

            // Assign the request
            available.remove(&assigned);
            busy.push(Reverse((finish_time, assigned)));
            count[assigned] += 1;
            max_requests = max_requests.max(count[assigned]);
        }

        // 4. Filter for all servers that achieved max_requests
        count
            .into_iter()
            .enumerate()
            .filter_map(|(server_id, c)| {
                if c == max_requests {
                    Some(server_id as i32)
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![1],
            Solution::busiest_servers(3, vec![1, 2, 3, 4, 5], vec![5, 2, 3, 3, 3])
        );
    }
}
