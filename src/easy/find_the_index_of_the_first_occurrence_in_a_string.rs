//! # Find the Index of the First Occurrence in a String
//!
//! Given two strings needle and haystack, return the index of the first
//! occurrence of needle in haystack, or -1 if needle is not part of
//! haystack.
//!
//! ```
//! # use leetcode::easy;
//! # use easy::find_the_index_of_the_first_occurrence_in_a_string;
//! # use find_the_index_of_the_first_occurrence_in_a_string::Solution;
//! let haystack = "sadbutsad".to_string();
//! let needle = "sad".to_string();
//! assert_eq!(Solution::str_str(haystack, needle), 0);
//!
//! let haystack = "leetcode".to_string();
//! let needle = "leeto".to_string();
//! assert_eq!(Solution::str_str(haystack, needle), -1);
//! ```

pub struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }

        let haystack = haystack.into_bytes();
        let needle = needle.into_bytes();

        if needle.len() > haystack.len() {
            return -1;
        }

        let word_size = needle.len();
        let stack_size = haystack.len();
        let limit = stack_size.saturating_sub(word_size);
        for index in 0..=limit {
            let start = index;
            let end = index + word_size;
            if haystack[start..end] == needle {
                return start as i32;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let haystack = "sadbutsad".to_string();
        let needle = "sad".to_string();
        let expected = 0;
        let result = Solution::str_str(haystack, needle);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let haystack = "leetcode".to_string();
        let needle = "leeto".to_string();
        let expected = -1;
        let result = Solution::str_str(haystack, needle);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_3() {
        let haystack = "leetcode".to_string();
        let needle = "code".to_string();
        let expected = 4;
        let result = Solution::str_str(haystack, needle);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_4() {
        let haystack = "aaa".to_string();
        let needle = "aaaaa".to_string();
        let expected = -1;
        let result = Solution::str_str(haystack, needle);
        assert_eq!(result, expected);
    }
}
