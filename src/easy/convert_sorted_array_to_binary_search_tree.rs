//! # Convert Sorted Array to Binary Search Tree
//!
//! Given an integer array `numbers` where the elements are sorted in ascending
//! order, convert it to a height balanced binary search tree.
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
    /// Given an integer array `numbers`` where the elements are sorted in
    /// ascending order, convert it to a binary search tree.
    pub fn sorted_array_to_bst(numbers: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::recurse(numbers.as_slice())
    }

    fn recurse(array: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        match array.len() {
            0 => None,
            1 => {
                let val = array[0];
                let tree = TreeNode::new(val);
                let refcell = RefCell::new(tree);
                let rc = Rc::new(refcell);
                Some(rc)
            }
            n => {
                let left = &array[0..n / 2];
                let mut right = &array[n / 2..n];
                let middle = right[0];
                if right.len() == 1 {
                    right = &[];
                } else {
                    right = &right[1..right.len()];
                }

                let mut tree = TreeNode::new(middle);
                tree.left = Self::recurse(left);
                tree.right = Self::recurse(right);
                Some(Rc::new(RefCell::new(tree)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let numbers = vec![];
        let output = Solution::sorted_array_to_bst(numbers);
        let expected = None;
        assert_eq!(output, expected, "expected {expected:?}, found {output:?}");
    }

    #[test]
    fn test_case_2() {
        let numbers = vec![0];
        let output = Solution::sorted_array_to_bst(numbers);
        let expected = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        assert_eq!(output, expected, "expected {expected:?}, found {output:?}");
    }

    #[test]
    fn test_case_3() {
        let numbers = vec![0, 1];
        let output = Solution::sorted_array_to_bst(numbers);
        let mut tree = TreeNode::new(1);
        tree.left = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        let expected = Some(Rc::new(RefCell::new(tree)));
        assert_eq!(output, expected, "expected {expected:?}, found {output:?}");
    }

    #[test]
    fn test_case_4() {
        let numbers = vec![0, 1, 2];
        let output = Solution::sorted_array_to_bst(numbers);
        let mut tree = TreeNode::new(1);
        tree.left = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        tree.right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let expected = Some(Rc::new(RefCell::new(tree)));
        assert_eq!(output, expected, "expected {expected:?}, found {output:?}");
    }

    #[test]
    fn test_case_5() {
        let numbers = vec![0, 1, 2, 3];
        let output = Solution::sorted_array_to_bst(numbers);
        let expected = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                right: None,
                left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            }))),
        })));

        assert_eq!(output, expected, "expected {expected:?}, found {output:?}");
    }

    #[test]
    fn test_case_6() {
        let numbers = vec![0, 1, 2, 3, 4];
        let output = Solution::sorted_array_to_bst(numbers);
        let expected = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: None,
            }))),
        })));

        assert_eq!(output, expected, "expected {expected:?}, found {output:?}");
    }
}
