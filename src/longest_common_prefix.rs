//! # Longest Common Prefix
//!
//! Write a function to find the longest common prefix string amongst an array
//! of strings. If there is no common prefix, return an empty string "".
//!

pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(mut input: Vec<String>) -> String {
        let mut prefix = input.pop().unwrap_or_default().into_bytes();

        'outer: for string in input {
            let chars = string.into_bytes();
            for index in 0..prefix.len() {
                let char1 = prefix
                    .get(index)
                    .expect("index only goes till prefix.len()");
                match (char1, chars.get(index)) {
                    (char1, Some(char2)) if char1 == char2 => continue,
                    _ => {
                        prefix.truncate(index);
                        continue 'outer;
                    }
                }
            }
        }

        String::from_utf8(prefix).unwrap()
    }
}
