use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub struct Solution {}

impl Solution {
    pub fn assign_tasks(servers: Vec<i32>, tasks: Vec<i32>) -> Vec<i32> {
        let m = tasks.len();
        let mut ans = Vec::with_capacity(m);

        // Min-heap for busy servers: Reverse((free_time, server_index))
        let mut busy: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();

        // Min-heap for idle servers: Reverse((weight, server_index))
        let mut idle: BinaryHeap<Reverse<(i32, usize)>> = servers
            .iter()
            .enumerate()
            .map(|(i, &w)| Reverse((w, i)))
            .collect();

        let mut ts: i64 = 0;

        for (i, &task_duration) in tasks.iter().enumerate() {
            ts = ts.max(i as i64);
            Self::release(&mut busy, &mut idle, &servers, ts);

            if idle.is_empty() {
                if let Some(&Reverse((next_free_ts, _))) = busy.peek() {
                    ts = next_free_ts;
                    Self::release(&mut busy, &mut idle, &servers, ts);
                }
            }

            if let Some(Reverse((_, idx))) = idle.pop() {
                ans.push(idx as i32);
                busy.push(Reverse((ts + task_duration as i64, idx)));
            }
        }

        ans
    }

    fn release(
        busy: &mut BinaryHeap<Reverse<(i64, usize)>>,
        idle: &mut BinaryHeap<Reverse<(i32, usize)>>,
        servers: &[i32],
        ts: i64,
    ) {
        while let Some(&Reverse((free_ts, idx))) = busy.peek() {
            if free_ts <= ts {
                busy.pop();
                idle.push(Reverse((servers[idx], idx)));
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![2,2,0,2,1,2],
            Solution::assign_tasks(
                vec![3,3,2],
                vec![1,2,3,2,1,2]
            )
        );
    }
}
