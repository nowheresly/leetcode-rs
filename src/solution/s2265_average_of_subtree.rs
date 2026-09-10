// Definition for a binary tree node.
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }
}

pub struct Solution {}

impl Solution {
    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut count = 0;
        Self::dfs(&root, &mut count);
        count
    }

    // Returns a tuple: (subtree_sum, subtree_count)
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, count: &mut i32) -> (i32, i32) {
        match node {
            None => (0, 0),
            Some(n) => {
                let borrowed = n.borrow();

                let (left_sum, left_count) = Self::dfs(&borrowed.left, count);
                let (right_sum, right_count) = Self::dfs(&borrowed.right, count);

                let total_sum = borrowed.val + left_sum + right_sum;
                let total_count = 1 + left_count + right_count;

                // Integer division in Rust automatically truncates towards zero
                if borrowed.val == total_sum / total_count {
                    *count += 1;
                }

                (total_sum, total_count)
            }
        }
    }

}

// Helper to construct a tree from a LeetCode-style level-order slice
pub fn to_tree(values: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
    if values.is_empty() || values[0].is_none() {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(values[0].unwrap())));
    let mut queue = VecDeque::new();
    queue.push_back(Rc::clone(&root));

    let mut i = 1;
    while i < values.len() {
        if let Some(curr) = queue.pop_front() {
            // Left child
            if i < values.len() {
                if let Some(val) = values[i] {
                    let left_node = Rc::new(RefCell::new(TreeNode::new(val)));
                    curr.borrow_mut().left = Some(Rc::clone(&left_node));
                    queue.push_back(left_node);
                }
                i += 1;
            }

            // Right child
            if i < values.len() {
                if let Some(val) = values[i] {
                    let right_node = Rc::new(RefCell::new(TreeNode::new(val)));
                    curr.borrow_mut().right = Some(Rc::clone(&right_node));
                    queue.push_back(right_node);
                }
                i += 1;
            }
        }
    }

    Some(root)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // [4, 8, 5, 0, 1, null, 6]
        let root = to_tree(&[
            Some(4),
            Some(8),
            Some(5),
            Some(0),
            Some(1),
            None,
            Some(6),
        ]);
        assert_eq!(
            5,
            Solution::average_of_subtree( root)
        );
    }

}
