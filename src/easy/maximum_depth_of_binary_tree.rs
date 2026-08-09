//! # Maximum Depth of Binary Tree
//!
//! Given the root of a binary tree, return its maximum depth.
//!
//! A binary tree's maximum depth is the number of nodes along the longest path
//! from the root node down to the farthest leaf node.
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
    /// Given the root of a binary tree, return its maximum depth.
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let level = 1;
                let root = node.borrow_mut().left.take();
                let left = Self::recurse(root, level);
                let root = node.borrow_mut().right.take();
                let right = Self::recurse(root, level);
                std::cmp::max(left, right)
            }
        }
    }

    fn recurse(root: Option<Rc<RefCell<TreeNode>>>, level: i32) -> i32 {
        match root {
            None => level,
            Some(node) => {
                let level = level + 1;
                let root = node.borrow_mut().left.take();
                let left = Self::recurse(root, level);
                let root = node.borrow_mut().right.take();
                let right = Self::recurse(root, level);
                std::cmp::max(left, right)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let output = Solution::max_depth(None);
        let expected = 0;
        assert_eq!(output, expected, "expected {expected}, found {output}");
    }

    #[test]
    fn test_case_2() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    right: None,
                    left: None,
                }))),
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 15,
                    right: None,
                    left: None,
                }))),
            }))),
        })));

        let output = Solution::max_depth(tree);
        let expected = 3;
        assert_eq!(output, expected, "expected {expected}, found {output}");
    }

    #[test]
    fn test_case_3() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                right: None,
                left: None,
            }))),
        })));

        let output = Solution::max_depth(tree);
        let expected = 2;
        assert_eq!(output, expected, "expected {expected}, found {output}");
    }
}
