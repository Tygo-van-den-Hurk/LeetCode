//! # Balanced Binary Tree
//!
//! Given a binary tree, determine if it is height balanced.
//!

use std::cell::RefCell;
use std::ops::Sub;
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
    /// Given the root of a binary tree, return if its height is balanced.
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            None => true,
            Some(node) => {
                let (_, balanced) = Self::recurse(node);
                balanced
            }
        }
    }

    fn recurse(root: Rc<RefCell<TreeNode>>) -> (i32, bool) {
        let left = root.borrow_mut().left.take();
        let right = root.borrow_mut().right.take();
        match (left, right) {
            (None, None) => (0, true),
            (None, Some(node)) | (Some(node), None) => {
                let (recursed_depth, recursed_balanced) = Self::recurse(node);
                let depth = recursed_depth + 1;
                let balanced = recursed_balanced && depth.le(&1);
                (depth, balanced)
            }
            (Some(left), Some(right)) => {
                let (left_depth, left_balanced) = Self::recurse(left);
                if !left_balanced {
                    return (0, false);
                }

                let (right_depth, right_balanced) = Self::recurse(right);
                if !right_balanced {
                    return (0, false);
                }

                let depth = 1 + std::cmp::max(left_depth, right_depth);
                let balanced = left_balanced && right_balanced;
                let balanced = balanced && left_depth.sub(right_depth).abs().le(&1);
                (depth, balanced)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_tree() {
        let expected = true;
        let output = Solution::is_balanced(None);
        assert_eq!(output, expected, "expected {expected}, found {output}");
    }

    #[test]
    fn only_leaf() {
        let tree = TreeNode::new(0);
        let tree = Some(Rc::new(RefCell::new(tree)));
        let output = Solution::is_balanced(tree);
        let expected = true;
        assert_eq!(output, expected, "expected {expected}, found {output}");
    }

    #[test]
    fn full_node_with_only_leafs() {
        let mut tree = TreeNode::new(0);
        tree.left = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        tree.right = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        let tree = Some(Rc::new(RefCell::new(tree)));

        let expected = true;
        let output = Solution::is_balanced(tree);
        assert_eq!(output, expected, "expected {expected}, found {output}");
    }

    #[test]
    fn full_tree_with_depth2() {
        let val = 0;

        let mut left = TreeNode::new(val);
        left.left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
        left.right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
        let left = Some(Rc::new(RefCell::new(left)));

        let mut right = TreeNode::new(val);
        right.left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
        right.right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
        let right = Some(Rc::new(RefCell::new(right)));

        let tree = TreeNode { val, left, right };
        let tree = Some(Rc::new(RefCell::new(tree)));

        let expected = true;
        let output = Solution::is_balanced(tree);
        assert_eq!(output, expected, "expected {expected}, found {output}");
    }

    #[test]
    fn left_1_behind_right() {
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

        let expected = true;
        let output = Solution::is_balanced(tree);
        assert_eq!(output, expected, "expected {expected}, found {output}");
    }

    #[test]
    fn left_2_behind_right() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 1_000,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2_100,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2_200,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3_210,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3_220,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4_221,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4_222,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
        })));

        let expected = false;
        let output = Solution::is_balanced(tree);
        assert_eq!(output, expected, "expected {expected}, found {output}");
    }

    #[test]
    fn almost_full_tree() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 1_000,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2_100,
                right: None,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3_110,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4_111,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4_112,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2_200,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3_210,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4_211,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4_212,
                        left: None,
                        right: None,
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3_220,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4_221,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4_222,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
        })));

        let output = Solution::is_balanced(tree);
        let expected = false;
        assert_eq!(output, expected, "expected {expected}, found {output}");
    }
}
