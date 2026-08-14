//! # Sort Vowels in a String
//!
//!  Given a string `input`, permute it to get a new string `output` such that:
//!
//!  - All consonants remain in their original places. More formally, if there
//!    is an index `i` with `0 <= i < input.len()` such that `input[i]` is a
//!    consonant, then `input[i] = output[i]`.
//!  - The vowels must be sorted in the non-decreasing order of their ASCII
//!    values. More formally, for pairs of indices `i`, `j` with:
//!    `0 <= i < j < input.len()` such that `input[i]` and `input[j]` are
//!    vowels, then `output[i]` must not have a higher ASCII value than
//!    `output[j]`.
//!
//! Return the resulting string.
//!
//! The vowels are `'a'`, `'e'`, `'i'`, `'o'`, and `'u'`, and they can appear
//! in lowercase or uppercase. Consonants comprise all letters that are not
//! vowels.
//!
//! ## Example
//!
//! ```
//! # use leetcode::easy::sort_vowels_in_a_string::Solution;
//! let input = "lEetcOde".to_string();
//! let expected = "lEOtcede".to_string();
//! let output = Solution::sort_vowels(input);
//! assert_eq!(output, expected);
//!
//! let input = "lYmpH".to_string();
//! let expected = "lYmpH".to_string();
//! let output = Solution::sort_vowels(input);
//! assert_eq!(output, expected);
//! ```
//!

pub struct Solution;

impl Solution {
    /// Sorts the vowels in the string, leaves any consonant in place.
    pub fn sort_vowels(input: String) -> String {
        let mut vowels = Vec::new();
        let mut consonants = Vec::new();
        let mut was_vowel = Vec::with_capacity(input.len());
        for character in input.chars() {
            match character {
                'a' | 'A' | 'e' | 'E' | 'i' | 'I' | 'o' | 'O' | 'u' | 'U' => {
                    was_vowel.push(true);
                    vowels.push(character);
                }
                character => {
                    was_vowel.push(false);
                    consonants.push(character);
                }
            }
        }

        vowels.sort();
        vowels.reverse();
        consonants.reverse();

        let mut result = String::with_capacity(input.len());
        for was_vowel in was_vowel {
            if was_vowel {
                let character = vowels.pop().unwrap();
                result.push(character);
            } else {
                let character = consonants.pop().unwrap();
                result.push(character);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "lEetcOde".to_string();
        let expected = "lEOtcede".to_string();
        let output = Solution::sort_vowels(input);
        assert_eq!(output, expected, "expected {expected}, received {output}");
    }

    #[test]
    fn test_case_2() {
        let input = "lYmpH".to_string();
        let expected = "lYmpH".to_string();
        let output = Solution::sort_vowels(input);
        assert_eq!(output, expected, "expected {expected}, received {output}");
    }
}
