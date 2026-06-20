pub struct Solution {}

impl Solution {
    pub fn max_building(n: i32, restrictions: Vec<Vec<i32>>) -> i32 {
        let mut reqs = Vec::with_capacity(restrictions.len() + 2);
        reqs.push((1, 0));
        for res in restrictions.iter() {
            reqs.push((res[0], res[1]));
        }
        reqs.sort_unstable();
        if reqs.last().unwrap().0 != n {
            reqs.push((n, n - 1));
        }
        let m = reqs.len();

        for i in 1..m {
            let prev_id = reqs[i-1].0;
            let prev_h = reqs[i-1].1;
            let curr_id = reqs[i].0;
            let curr_h = reqs[i].1;
            reqs[i] = (curr_id, curr_h.min(prev_h + (curr_id - prev_id)));
        }

        for i in (0..(m-1)).rev() {
            let next_id = reqs[i+1].0;
            let next_h = reqs[i+1].1;
            let curr_id = reqs[i].0;
            let curr_h = reqs[i].1;
            reqs[i] = (curr_id, curr_h.min(next_h + (next_id - curr_id)));
        }

        let mut max_h = 0;
        for i in 1..m {
            let (id1, h1) = reqs[i-1];
            let (id2, h2) = reqs[i];

            let peak = (h1 + h2 + (id2 - id1)) / 2;
            max_h = max_h.max(peak);
        }

        max_h
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            5,
            Solution::max_building(10, vec![vec![5, 3], vec![2, 5], vec![7, 4], vec![10, 3]])
        );
    }
}
