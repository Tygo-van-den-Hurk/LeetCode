//! # Jewels and Stones
//!
//! You're given strings jewels representing the types of stones that are
//! jewels, and stones representing the stones you have. Each character in
//! stones is a type of stone you have. You want to know how many of the
//! stones you have are also jewels.
//!
//! Letters are case sensitive, so "a" is considered a different type of stone
//! from "A".
//!

pub struct Solution;

impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        type Set = std::collections::HashSet<char>;
        let set = jewels.chars().collect::<Set>();
        stones.chars().fold(0, |total, stone| {
            if set.contains(&stone) {
                total + 1
            } else {
                total
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let jewels = "aA".to_string();
        let stones = "aAAbbbb".to_string();
        let output = Solution::num_jewels_in_stones(jewels, stones);
        let expected = 3;
        assert_eq!(output, expected);
    }

    #[test]
    fn test_case_2() {
        let jewels = "z".to_string();
        let stones = "ZZ".to_string();
        let output = Solution::num_jewels_in_stones(jewels, stones);
        let expected = 0;
        assert_eq!(output, expected);
    }
}
