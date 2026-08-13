pub struct Solution {}

#[derive(Clone, Debug)]
pub struct Node {
    max: i32,
    pre: i32,
    suf: i32,
    size: i32,
    first: char,
    last: char,
}

impl Node {
    fn new() -> Self {
        Self {
            max: 0,
            pre: 0,
            suf: 0,
            size: 0,
            first: 'a',
            last: 'a',
        }
    }

    fn merge(&mut self, left: &Self, right: &Self) {
        self.size = left.size + right.size;
        self.first = left.first;
        self.last = right.last;

        self.pre = left.pre;
        self.suf = right.suf;
        self.max = i32::max(left.max, right.max);

        if left.last == right.first {
            self.max = self.max.max(left.suf + right.pre);

            if left.pre == left.size {
                self.pre = left.pre + right.pre;
            }

            if right.suf == right.size {
                self.suf = right.size + left.suf;
            }
        }
    }
}

pub struct SegmentTree {
    pub tree: Vec<Node>,
}

impl SegmentTree {
    pub fn build(&mut self, node: usize, l: usize, r: usize, s: &Vec<char>) {
        if l == r {
            let mut n = Node::new();
            n.first = s[l];
            n.last = s[r];
            n.size = 1;
            n.pre = 1;
            n.suf = 1;
            n.max = 1;
            self.tree[node] = n;
            return;
        }
        let mid = (l + r) / 2;
        self.build(2 * node, l, mid, s);
        self.build(2 * node + 1, mid + 1, r, s);

        let mut n = Node::new();
        n.merge(&self.tree[2 * node], &self.tree[2 * node + 1]);
        self.tree[node] = n;
    }

    pub fn update(&mut self, node: usize, l: usize, r: usize, idx: usize, val: char) {
        if l == r {
            self.tree[node].first = val;
            self.tree[node].last = val;
            return;
        }

        let mid = (l + r) / 2;

        if l <= idx && idx <= mid {
            self.update(2 * node, l, mid, idx, val);
        } else {
            self.update(2 * node + 1, mid + 1, r, idx, val);
        }
        let mut n = Node::new();
        n.merge(&self.tree[2 * node], &self.tree[2 * node + 1]);
        self.tree[node] = n;
    }
}

impl SegmentTree {
    pub fn new(n: usize) -> Self {
        Self {
            tree: vec![Node::new(); 4 * n],
        }
    }
}

impl Solution {
    pub fn longest_repeating(
        s: String,
        query_characters: String,
        query_indices: Vec<i32>,
    ) -> Vec<i32> {
        let k = query_indices.len();
        let mut res = vec![0; k];
        let n = s.len();

        let s: Vec<char> = s.chars().collect();
        let mut st = SegmentTree::new(n);
        st.build(1, 0, n - 1, &s);

        let qc: Vec<char> = query_characters.chars().collect();

        for i in 0..k {
            let val = qc[i];
            let idx = query_indices[i] as usize;
            st.update(1, 0, n - 1, idx, val);
            res[i] = st.tree[1].max;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![3, 3, 4],
            Solution::longest_repeating(String::from("babacc"), String::from("bcb"), vec![1, 3, 3])
        );
    }
}
