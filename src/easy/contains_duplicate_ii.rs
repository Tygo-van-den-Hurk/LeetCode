//! # Contains Duplicate II
//!
//! Given an integer array `numbers` and an integer `window_size`, return
//! `true` if there are two distinct indices `i` and `j` in the array such
//! that `numbers[i] == numbers[j]` and `abs(i - j) <= window_size`.
//!

pub struct Solution;

impl Solution {
    pub fn contains_nearby_duplicate(numbers: Vec<i32>, window_size: i32) -> bool {
        use std::collections::HashSet;

        let window_size = window_size as usize;
        let capacity = window_size + 1;
        let mut window = HashSet::with_capacity(capacity);
        for index in 0..numbers.len() {
            if window.contains(&numbers[index]) {
                return true;
            }

            window.insert(numbers[index]);

            if window.len() > window_size {
                let too_far_away = index - window_size;
                window.remove(&numbers[too_far_away]);
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let numbers = vec![1, 2, 3, 1];
        let window_size = 3;
        let output = Solution::contains_nearby_duplicate(numbers, window_size);
        let expected = true;
        assert_eq!(output, expected);
    }

    #[test]
    fn test_case_2() {
        let numbers = vec![1, 0, 1, 1];
        let window_size = 1;
        let output = Solution::contains_nearby_duplicate(numbers, window_size);
        let expected = true;
        assert_eq!(output, expected);
    }

    #[test]
    fn test_case_3() {
        let numbers = vec![1, 2, 3, 1, 2, 3];
        let window_size = 2;
        let output = Solution::contains_nearby_duplicate(numbers, window_size);
        let expected = false;
        assert_eq!(output, expected);
    }
}
