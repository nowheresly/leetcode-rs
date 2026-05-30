use std::collections::BTreeSet;

pub struct Solution {}

impl Solution {
    pub fn get_results(queries: Vec<Vec<i32>>) -> Vec<bool> {
        let max_x = queries.iter().map(|q| q[1]).max().unwrap_or(0) + 1;
        let mut st = BTreeSet::new();
        st.insert(0);
        st.insert(max_x);

        for q in queries.iter() {
            if q[0] == 1 {
                st.insert(q[1]);
            }
        }

        let mut bt = vec![0; max_x as usize + 1];

        let mut pre = 0;

        for x in st.iter() {
            if *x == 0 {
                continue;
            }
            update(&mut bt, *x, *x - pre);
            pre = *x;
        }

        let mut res = Vec::new();
        for i in (0..queries.len()).rev() {
            let q = &queries[i];
            if q[0] == 2 {
                let x = q[1];
                let sz = q[2];
                let pre_val = *st.range(..=x).last().unwrap_or(&0);
                let max_space_before_pre = query(&bt, pre_val);
                let max_space = max_space_before_pre.max(x - pre_val);

                res.push(max_space >= sz);
            } else {
                let x = q[1];
                let pre_val = *st.range(..x).last().unwrap_or(&0);
                let nxt = *st.range((x + 1)..).next().unwrap_or(&max_x);
                update(&mut bt, nxt, nxt - pre_val);
                st.remove(&x);
            }
        }
        res.reverse();
        res
    }
}

fn query(bt: & Vec<i32>, x: i32) -> i32 {
    let mut res = 0;
    let mut x = x;
    while x > 0 {
        res = res.max(bt[x as usize]);
        x -= x & -x;
    }
    res
}

fn update(bt: &mut Vec<i32>, x: i32, v: i32) {
    let mut x = x;
    while x < bt.len() as i32 {
        bt[x as usize] = v.max(bt[x as usize]);
        x += x & -x;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![false, true, true],
            Solution::get_results(vec![
                vec![1, 2],
                vec![2, 3, 3],
                vec![2, 3, 1],
                vec![2, 2, 2]
            ])
        );
    }
}
