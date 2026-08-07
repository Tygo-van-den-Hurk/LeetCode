//! # Merge Two Sorted Lists
//!
//! You are given the heads of two sorted linked lists `list1` and `list2`.
//! Merge the two lists into one sorted list. The list should be made by
//! splicing together the nodes of the first two lists. Return the head of
//! the merged linked list.
//!
//! <center><br>
//!   <img src="https://assets.leetcode.com/uploads/2020/10/03/merge_ex1.jpg"
//!     alt="example solution">
//! </center>
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
    pub fn merge_two_lists(
        mut option1: Option<Box<ListNode>>,
        mut option2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result = ListNode::new(0);

        match (&mut option1, &mut option2) {
            (None, None) => return None,
            (Some(node), None) | (None, Some(node)) => {
                result.val = node.val;
                result.next = node.next.take();
                return Some(Box::new(result));
            }
            (Some(node1), Some(node2)) => {
                if node1.val < node2.val {
                    result.val = node1.val;
                    option1 = node1.next.take();
                } else {
                    result.val = node2.val;
                    option2 = node2.next.take();
                }
            }
        };

        result.next = Self::merge_two_lists(option1, option2);
        Some(Box::new(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let list1 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));

        let list2 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));

        let result = Solution::merge_two_lists(list1, list2);
        let expected = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode { val: 4, next: None })),
                        })),
                    })),
                })),
            })),
        }));

        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let list1 = None;
        let list2 = None;
        let result = Solution::merge_two_lists(list1, list2);
        let expected = None;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_3() {
        let list1 = None;
        let list2 = Some(Box::new(ListNode { val: 0, next: None }));

        let result = Solution::merge_two_lists(list1, list2);
        let expected = Some(Box::new(ListNode { val: 0, next: None }));

        assert_eq!(result, expected);
    }
}
