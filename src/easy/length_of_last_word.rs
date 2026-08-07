//!  # Length of Last Word
//!
//! Given a string `s` consisting of words and spaces, return the length of
//! the last word in the string. A word is a maximal consisting of non-space
//! characters only.
//!
//! ## Examples
//!
//! ```
//! # use leetcode::easy::length_of_last_word::Solution;
//!
//! let input = "Hello World".to_string();
//! let expected = 5;
//! let result = Solution::length_of_last_word(input);
//! assert_eq!(result, expected);
//!
//! let input = "   fly me   to   the moon  ".to_string();
//! let expected = 4;
//! let result = Solution::length_of_last_word(input);
//! assert_eq!(result, expected);
//!
//! let input = "luffy is still joyboy".to_string();
//! let expected = 6;
//! let result = Solution::length_of_last_word(input);
//! assert_eq!(result, expected);
//! ```
//!

pub struct Solution;

impl Solution {
    pub fn length_of_last_word(input: String) -> i32 {
        let mut input = input.chars().rev();

        // ignore the whitespace at the end
        for character in &mut input {
            if !character.is_whitespace() {
                break;
            }
        }

        // now that we've read 1 non-whitespace character, we count
        let mut size = 1;
        for character in input {
            if !character.is_whitespace() {
                size += 1;
            } else {
                break;
            }
        }

        return size;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "Hello World".to_string();
        let expected = 5;
        let result = Solution::length_of_last_word(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let input = "   fly me   to   the moon  ".to_string();
        let expected = 4;
        let result = Solution::length_of_last_word(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_3() {
        let input = "luffy is still joyboy".to_string();
        let expected = 6;
        let result = Solution::length_of_last_word(input);
        assert_eq!(result, expected);
    }
}
