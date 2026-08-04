//! # Search Insert Position
//!
//! Given a sorted array of distinct integers and a target value, return the
//! index if the target is found. If not, return the index where it would be if
//! it were inserted in order. You must write an algorithm with O(log n)
//! runtime complexity.
//!

pub struct Solution;

impl Solution {
    pub fn search_insert(numbers: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = numbers.len();

        while left < right {
            let middle = left + (right - left) / 2;
            if numbers[middle] == target {
                return middle as i32;
            } else if numbers[middle] < target {
                left = middle + 1;
            } else {
                right = middle;
            }
        }

        left as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = vec![1, 3, 5, 6];
        let target = 5;
        let output = Solution::search_insert(input, target);
        let expected = 2;
        assert_eq!(output, expected);
    }

    #[test]
    fn test_case_2() {
        let input = vec![1, 3, 5, 6];
        let target = 2;
        let output = Solution::search_insert(input, target);
        let expected = 1;
        assert_eq!(output, expected);
    }

    #[test]
    fn test_case_3() {
        let input = vec![1, 3, 5, 6];
        let target = 7;
        let output = Solution::search_insert(input, target);
        let expected = 4;
        assert_eq!(output, expected);
    }

    #[test]
    fn test_case_4() {
        let input = vec![1, 3, 5, 6];
        let target = 1;
        let output = Solution::search_insert(input, target);
        let expected = 0;
        assert_eq!(output, expected);
    }

    #[test]
    fn test_case_5() {
        let input = vec![1, 3, 5, 6];
        let target = 4;
        let output = Solution::search_insert(input, target);
        let expected = 2;
        assert_eq!(output, expected);
    }
}
