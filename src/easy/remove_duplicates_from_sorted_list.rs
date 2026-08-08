//! # Remove Duplicates from Sorted List
//!
//! Given the head of a sorted linked list, delete all duplicates such that
//! each element appears only once. Return the linked list sorted as well.
//!
//! ```
//! # use leetcode::easy::remove_duplicates_from_sorted_list::ListNode;
//! # use leetcode::easy::remove_duplicates_from_sorted_list::Solution;
//! assert_eq!(Solution::delete_duplicates(None), None);
//!
//! let input = Some(Box::new(ListNode {
//!     val: 1,
//!     next: Some(Box::new(ListNode {
//!         val: 1,
//!         next: Some(Box::new(ListNode {
//!             val: 2,
//!             next: Some(Box::new(ListNode {
//!                 val: 3,
//!                 next: Some(Box::new(ListNode { val: 3, next: None })),
//!             })),
//!         })),
//!     })),
//! }));
//!
//! let output = Solution::delete_duplicates(input);
//! let expected = Some(Box::new(ListNode {
//!     val: 1,
//!     next: Some(Box::new(ListNode {
//!         val: 2,
//!         next: Some(Box::new(ListNode { val: 3, next: None })),
//!     })),
//! }));
//!
//! assert_eq!(output, expected);
//! ```
//!

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution;

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut list = Vec::new();
        while let Some(mut node) = head {
            head = node.next.take();
            let val = node.val;
            if Some(&val) != list.last() {
                list.push(val);
            }
        }

        list.into_iter()
            .rev()
            .fold(None, |next, val| Some(Box::new(ListNode { next, val })))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode { val: 2, next: None })),
            })),
        }));

        let output = Solution::delete_duplicates(input);
        let expected = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None })),
        }));

        assert_eq!(output, expected);
    }

    #[test]
    fn test_case_2() {
        let input = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode { val: 3, next: None })),
                    })),
                })),
            })),
        }));

        let output = Solution::delete_duplicates(input);
        let expected = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        }));

        assert_eq!(output, expected);
    }

    #[test]
    fn test_case_3() {
        let input = None;
        let output = Solution::delete_duplicates(input);
        let expected = None;
        assert_eq!(output, expected);
    }
}
