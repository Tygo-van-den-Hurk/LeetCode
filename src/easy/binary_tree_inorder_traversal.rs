//! # Binary Tree Inorder Traversal
//!
//! Given the root of a binary tree, return the inorder traversal of its
//! nodes' values.

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

    /// Given the root of a binary tree, return the inorder traversal of its
    /// nodes' values.
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        Self::recurse(root, &mut result);
        result
    }

    /// Recurses the `TreeNode`s left to right.
    fn recurse(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(node) = root {
            Self::recurse(node.borrow_mut().left.take(), result);
            result.push(node.borrow_mut().val);
            Self::recurse(node.borrow_mut().right.take(), result);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                right: None,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
            }))),
        })));

        let output = Solution::inorder_traversal(input);
        let expected = vec![1, 3, 2];
        assert_eq!(output, expected);
    }

    #[test]
    fn test_case_2() {
        let input = None;
        let output = Solution::inorder_traversal(input);
        let expected = vec![];
        assert_eq!(output, expected);
    }

    #[test]
    fn test_case_3() {
        let input = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 6,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 8,
                    right: None,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 9,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
        })));

        let output = Solution::inorder_traversal(input);
        let expected = vec![4, 2, 6, 5, 7, 1, 3, 9, 8];
        assert_eq!(output, expected);
    }
}
