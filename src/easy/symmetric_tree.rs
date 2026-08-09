//! # Symmetric Tree
//!
//! Given the root of a binary tree, check whether it is a mirror of itself
//! (i.e., symmetric around its center).
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
    /// Given the root of a binary tree, checks whether it is a mirror of
    /// itself (i.e., symmetric around its center).
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            None => true,
            Some(node) => {
                let left = node.borrow_mut().left.take();
                let right = node.borrow_mut().right.take();
                Self::recurse(left, right)
            }
        }
    }

    fn recurse(tree1: Option<Rc<RefCell<TreeNode>>>, tree2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (tree1, tree2) {
            (Some(_), None) | (None, Some(_)) => false,
            (None, None) => true,
            (Some(node1), Some(node2)) => {
                node1.borrow().val == node2.borrow().val
                    && Self::recurse(
                        node1.borrow_mut().left.take(),
                        node2.borrow_mut().right.take(),
                    )
                    && Self::recurse(
                        node1.borrow_mut().right.take(),
                        node2.borrow_mut().left.take(),
                    )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
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

        assert!(!Solution::is_symmetric(tree));
    }

    #[test]
    fn test_case_2() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                right: None,
                left: None,
            }))),
        })));

        assert!(!Solution::is_symmetric(tree));
    }

    #[test]
    fn test_case_3() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    right: None,
                    left: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    right: None,
                    left: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    right: None,
                    left: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    right: None,
                    left: None,
                }))),
            }))),
        })));

        assert!(Solution::is_symmetric(tree));
    }

    #[test]
    fn test_case_4() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    right: None,
                    left: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    right: None,
                    left: None,
                }))),
                right: None,
            }))),
        })));

        assert!(Solution::is_symmetric(tree));
    }

    #[test]
    fn test_case_5() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    right: None,
                    left: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    right: None,
                    left: None,
                }))),
                right: None,
            }))),
        })));

        assert!(!Solution::is_symmetric(tree));
    }
}
