use std::collections::HashMap;

pub struct Solution {
}

pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>
}

impl UnionFind {
    fn find(self: &mut Self, node: usize) -> usize {
        if self.parent[node] == node {
            return node;
        }
        self.parent[node] = self.find(self.parent[node]);
        self.parent[node]
    }

    fn union(self: &mut Self, node1: usize, node2: usize) {
        let root1 = self.find(node1);
        let root2 = self.find(node2);
        if root1 == root2 {
            return;
        }
        if self.rank[root1]>self.rank[root2] {
            self.parent[root2] = root1;
        } else if self.rank[root1]<self.rank[root2] {
            self.parent[root1] = root2;
        } else {
            self.parent[root2] = root1;
            self.rank[root1] += 1;
        }
    }
}

impl Solution {
    pub fn minimum_hamming_distance(source: Vec<i32>, target: Vec<i32>, allowed_swaps: Vec<Vec<i32>>) -> i32 {
        let n = source.len();
        let mut uf = UnionFind {
            parent: vec![0;n],
            rank: vec![0;n]
        };
        for i in 0..n {
            uf.parent[i] = i;
        }
        for swap in allowed_swaps {
            uf.union(swap[0] as usize, swap[1] as usize);
        }
        let mut comp_count:HashMap<usize, HashMap<usize, i32>> = HashMap::new();
        for i in 0..n {
            let root = uf.find(i);
            let counts = comp_count.entry(root).or_insert(HashMap::new());
            *counts.entry(source[i] as usize).or_insert(0) += 1;
        }
        let mut res = 0;
        for i in 0..n {
            let root = uf.find(i);
            let counts = comp_count.get_mut(&root).unwrap();

            if *counts.entry(target[i] as usize).or_insert(0) > 0 {
                *counts.entry(target[i] as usize).or_insert(0) -= 1;
            } else {
                res += 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::minimum_hamming_distance(vec![1,2,3,4], vec![2,1,4,5], vec![vec![0,1],vec![2,3]]));

    }
}
