

struct TreeAncestor {
    up: Vec<Vec<i32>>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TreeAncestor {

    fn new(n: i32, parent: Vec<i32>) -> Self {
        let mut up = vec![vec![-1; 16];n as usize];
        for i in 0..n {
            up[i as usize][0] = parent[i as usize];
        }
        for j in 1..16 {
            for i in 0..parent.len() {
                if up[i][j-1] == -1 {
                    continue;
                }
                up[i][j] = up[up[i][j-1] as usize][j-1];
            }
        }
        Self {
            up
        }
    }

    fn get_kth_ancestor(&self, node: i32, k: i32) -> i32 {
        let mut node = node;
        for j in 0..16 {
            if ((k >> j) & 1) == 0 {
                continue;
            }
            node = self.up[node as usize][j];
            if node == -1 {
                break;
            }
        }
        node
    }
}

/**
 * Your TreeAncestor object will be instantiated and called as such:
 * let obj = TreeAncestor::new(n, parent);
 * let ret_1: i32 = obj.get_kth_ancestor(node, k);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let tree = TreeAncestor::new(2, vec![4, 2, 1, 3]);
        assert_eq!(
            1,
            tree.get_kth_ancestor(3, 1)
        );

    }
}
