pub struct Solution {}

impl Solution {
    pub fn count_servers(n: i32, mut logs: Vec<Vec<i32>>, x: i32, queries: Vec<i32>) -> Vec<i32> {
        let q_len = queries.len();
        let mut ans = vec![0; q_len];

        // 1. Pair each query with its original index: (query_time, original_idx)
        let mut qs: Vec<(i32, usize)> = queries
            .into_iter()
            .enumerate()
            .map(|(idx, q)| (q, idx))
            .collect();

        // 2. Sort queries and logs by time
        qs.sort_unstable_by_key(|&(q, _)| q);
        logs.sort_unstable_by_key(|log| log[1]);

        // Frequency array for server IDs (1-indexed, up to n)
        let mut count = vec![0; (n + 1) as usize];
        let mut active_servers = 0;
        let mut l = 0;
        let mut r = 0;

        // 3. Drive the sliding window using each query's time interval
        for (end, original_idx) in qs {
            let start = end - x;

            // Expand right pointer: include all logs with time <= end
            while r < logs.len() && logs[r][1] <= end {
                let server_id = logs[r][0] as usize;
                if count[server_id] == 0 {
                    active_servers += 1;
                }
                count[server_id] += 1;
                r += 1;
            }

            // Shrink left pointer: remove all logs with time < start
            while l < logs.len() && logs[l][1] < start {
                let server_id = logs[l][0] as usize;
                count[server_id] -= 1;
                if count[server_id] == 0 {
                    active_servers -= 1;
                }
                l += 1;
            }

            // Inactive servers = Total servers - Active servers in current window
            ans[original_idx] = n - active_servers;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![0, 1],
            Solution::count_servers(
                3,
                vec![vec![2, 4], vec![2, 1], vec![1, 2], vec![3, 1]],
                2,
                vec![3, 4]
            )
        );
    }
}
