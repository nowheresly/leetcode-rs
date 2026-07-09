pub struct Solution {}

pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let parent = (0..n).collect();
        let rank = vec![0; n];
        Self { parent, rank }
    }
    fn find(&mut self, i: usize) -> usize {
        let j = self.parent[i];
        if i == j {
            i
        } else {
            self.parent[i] = self.find(self.parent[i]);
            self.parent[i]
        }
    }

    fn union(&mut self, x: usize, y: usize) {
        let px = self.find(x);
        let py = self.find(y);
        if px == py {
            return;
        }
        if self.rank[px] > self.rank[py] {
            self.parent[py] = px;
        } else if self.rank[px] < self.rank[py] {
            self.parent[px] = py;
        } else {
            // If ranks are equal, pick one as root and increment its rank
            self.parent[px] = py;
            self.rank[py] += 1;
        }
    }
}

impl Solution {
    pub fn path_existence_queries(n: i32, nums: Vec<i32>, max_diff: i32, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut graph = UnionFind::new(n as usize);
        for i in 1..nums.len() {
            if nums[i] - nums[i - 1] <= max_diff {
                graph.union(i, i-1);
            }
        }
        let mut res = vec![false; queries.len()];
        for i in 0..queries.len() {
            if graph.find(queries[i][0] as usize) == graph.find(queries[i][1] as usize) {
                res[i] = true;
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
        assert_eq!(vec![true, false], Solution::path_existence_queries(2, vec![1,3], 1, vec![vec![0,0],vec![0,1]]));
    }
}
