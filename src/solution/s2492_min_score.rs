
pub struct Solution {}

pub struct UF {
    rep: Vec<i32>
}

impl UF {
    fn new(n: i32) -> Self {
        let rep = (0..n).collect();
        UF { rep }
    }

    fn find(&mut self, x: i32) -> i32 {
        if self.rep[x as usize] != x {
            self.rep[x as usize] = self.find(self.rep[x as usize]);
        }
        self.rep[x as usize]
    }

    fn union(&mut self, x: i32, y: i32) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            self.rep[root_y as usize] = root_x;
        }
    }
}

impl Solution {
    pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let mut uf = UF::new(n + 1);
        let mut min = i32::MAX;
        for r in roads.iter() {
            uf.union(r[0], r[1]);
        }
        let root1 = uf.find(1);
        for r in roads.iter() {

            if uf.find(r[0]) == root1 {
                min = min.min(r[2]);
            }
        }
        min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::min_score(4, vec![vec![1,2,2],vec![1,3,4],vec![3,4,7]]));
    }
}
