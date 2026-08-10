//! # Path Sum
//!
//! Given the root of a binary tree and an integer `target`, return `true`
//! if the tree has a root-to-leaf path such that adding up all the values
//! along the path equals `target`.
//!
//! A leaf is a node with no children.
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
    /// # use leetcode::easy::symmetric_tree::TreeNode;
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
    /// Given the root of a binary tree and an integer `target`, return `true`
    /// if the tree has a root-to-leaf path such that adding up all the values
    /// along the path equals `target`.
    pub fn has_path_sum(node: Option<Rc<RefCell<TreeNode>>>, target: i32) -> bool {
        match node {
            Some(root) => Self::recurse(root, target),
            None => false,
        }
    }

    fn recurse(root: Rc<RefCell<TreeNode>>, target: i32) -> bool {
        let target = target - root.borrow().val;
        let left = root.borrow_mut().left.take();
        let right = root.borrow_mut().right.take();
        match (left, right) {
            (None, None) => target == 0,
            (Some(left), Some(right)) => {
                Self::recurse(left, target) || Self::recurse(right, target)
            }
            (None, Some(node)) | (Some(node), None) => Self::recurse(node, target),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let expected = false;
        let output = Solution::has_path_sum(None, 0);
        assert_eq!(output, expected, "expected {expected}, found {output}");
    }

    #[test]
    fn test_case_2() {
        let val = 12345;
        let expected = true;
        let tree = Some(Rc::new(RefCell::new(TreeNode::new(val))));
        let output = Solution::has_path_sum(tree, val);
        assert_eq!(output, expected, "expected {expected}, found {output}");
    }

    #[test]
    fn test_case_3() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                right: None,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 11,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        right: None,
                        left: None,
                    }))),
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        right: None,
                        left: None,
                    }))),
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 8,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        right: None,
                        left: None,
                    }))),
                    left: None,
                }))),
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 13,
                    right: None,
                    left: None,
                }))),
            }))),
        })));

        let target = 22;
        let output = Solution::has_path_sum(tree, target);
        let expected = true;
        assert_eq!(output, expected, "expected {expected}, found {output}");
    }

    #[test]
    fn test_case_4() {
        let target = 1;
        let mut tree = TreeNode::new(target);
        tree.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let tree = Some(Rc::new(RefCell::new(tree)));
        let output = Solution::has_path_sum(tree, target);
        let expected = false;
        assert_eq!(output, expected, "expected {expected}, found {output}");
    }
}
