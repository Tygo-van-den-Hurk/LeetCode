//! # Add Binary
//!
//! Given two binary strings a and b, return their sum as a binary string.
//!
//! ## Examples
//!
//! ```
//! # use leetcode::easy::add_binary::Solution;
//! let a = "11".to_string();
//! let b = "1".to_string();
//! let expected = "100".to_string();
//! let result = Solution::add_binary(a, b);
//! assert_eq!(result, expected);
//!
//! let a = "1010".to_string();
//! let b = "1011".to_string();
//! let expected = "10101".to_string();
//! let result = Solution::add_binary(a, b);
//! assert_eq!(result, expected);
//! ```
//!

pub struct Solution;

impl Solution {
    /// Converts a char into a boolean.
    fn to_bin(symbol: char) -> u8 {
        match symbol {
            '0' => 0,
            '1' => 1,
            _ => unreachable!("expected either '0' or '1', but found '{symbol}'"),
        }
    }

    /// Converts a boolean back into a char.
    fn to_char(symbol: u8) -> char {
        match symbol {
            0 => '0',
            1 => '1',
            _ => unreachable!("expected either '0' or '1', but found '{symbol}'"),
        }
    }

    pub fn add_binary(a: String, b: String) -> String {
        let max = 1 + std::cmp::max(a.len(), b.len());
        let mut sum = Vec::with_capacity(max);

        let mut a: Vec<u8> = a.chars().map(Self::to_bin).collect();
        let mut b: Vec<u8> = b.chars().map(Self::to_bin).collect();

        let mut remainder = 0;
        for _ in 0..max {
            let a = a.pop().unwrap_or_default();
            let b = b.pop().unwrap_or_default();
            match a + b + remainder {
                0 => {
                    sum.push(0);
                    remainder = 0;
                }
                1 => {
                    sum.push(1);
                    remainder = 0;
                }
                2 => {
                    sum.push(0);
                    remainder = 1;
                }
                3 => {
                    sum.push(1);
                    remainder = 1;
                }
                _ => unreachable!("three things that are at must 1 cannot sum over 3"),
            }
        }

        // Ensure there are no trailing zeros
        let result = sum.into_iter().map(Self::to_char);
        let result = result.rev().collect::<String>();
        let result = result.trim_start_matches('0');
        if result.is_empty() {
            "0".to_string()
        } else {
            String::from(result)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let a = "11".to_string();
        let b = "1".to_string();
        let expected = "100".to_string();
        let result = Solution::add_binary(a, b);
        assert_eq!(result, expected, "expected '{expected}' but got '{result}'");
    }

    #[test]
    fn test_case_2() {
        let a = "1010".to_string();
        let b = "1011".to_string();
        let expected = "10101".to_string();
        let result = Solution::add_binary(a, b);
        assert_eq!(result, expected, "expected '{expected}' but got '{result}'");
    }

    #[test]
    fn test_case_3() {
        let a = "0".to_string();
        let b = "0".to_string();
        let expected = "0".to_string();
        let result = Solution::add_binary(a, b);
        assert_eq!(result, expected, "expected '{expected}' but got '{result}'");
    }

    #[test]
    fn test_case_4() {
        let a = "1000".to_string();
        let b = "1000".to_string();
        let expected = "10000".to_string();
        let result = Solution::add_binary(a, b);
        assert_eq!(result, expected, "expected '{expected}' but got '{result}'");
    }
}
