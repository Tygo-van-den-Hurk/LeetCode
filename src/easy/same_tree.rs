//! # Same Tree
//!
//! Given the roots of two binary trees `tree1` and `tree2`, write a function
//! to check if they are the same or not.
//!
//! Two binary trees are considered the same if they are structurally identical,
//! and the nodes have the same value.
//!

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    /// Creates a new [`TreeNode`] with value `val`.
    ///
    /// ```
    /// # use leetcode::easy::binary_tree_inorder_traversal::TreeNode;
    /// let node = TreeNode::new(123);
    /// assert_eq!(node.val, 123);
    /// assert_eq!(node.left, None);
    /// assert_eq!(node.right, None);
    /// ```
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
pub struct Solution;

impl Solution {
    /// Given the roots of two binary trees `tree1` and `tree2`, checks if
    /// they are the same.
    /// ```
    /// # use leetcode::easy::same_tree::TreeNode;
    /// # use leetcode::easy::same_tree::Solution;
    /// # use std::cell::RefCell;
    /// # use std::rc::Rc;
    /// let tree1 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    /// let tree2 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    /// assert!(Solution::is_same_tree(tree1, tree2));
    ///
    /// let tree1 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    /// let tree2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    /// assert!(!Solution::is_same_tree(tree1, tree2));
    /// ```
    pub fn is_same_tree(
        tree1: Option<Rc<RefCell<TreeNode>>>,
        tree2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (tree1, tree2) {
            (Some(_), None) | (None, Some(_)) => false,
            (None, None) => true,
            (Some(node1), Some(node2)) => {
                if node1.borrow().val != node2.borrow().val {
                    return false;
                }

                let left1 = node1.borrow_mut().left.take();
                let left2 = node2.borrow_mut().left.take();
                if !Self::is_same_tree(left1, left2) {
                    return false;
                }

                let right1 = node1.borrow_mut().right.take();
                let right2 = node2.borrow_mut().right.take();
                Self::is_same_tree(right1, right2)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let tree1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                right: None,
                left: None,
            }))),
        })));

        let tree2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                right: None,
                left: None,
            }))),
        })));

        assert!(Solution::is_same_tree(tree1, tree2));
    }

    #[test]
    fn test_case_2() {
        let tree1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: None,
        })));

        let tree2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
        })));

        assert!(!Solution::is_same_tree(tree1, tree2));
    }

    #[test]
    fn test_case_3() {
        let tree1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                right: None,
                left: None,
            }))),
        })));

        let tree2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                right: None,
                left: None,
            }))),
        })));

        assert!(!Solution::is_same_tree(tree1, tree2));
    }
}
