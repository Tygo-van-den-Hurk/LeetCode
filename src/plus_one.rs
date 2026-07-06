//! # Plus One
//!
//! You are given a large integer represented as an integer array digits, where
//! each digits[i] is the ith digit of the integer. The digits are ordered from
//! most significant to least significant in left-to-right order. The large
//! integer does not contain any leading 0's. Increment the large integer by
//! one and return the resulting array of digits.
//!

pub struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        let digits = digits.into_iter().rev();
        let digits = digits.collect::<Vec<i32>>();
        for index in 0..digits.len() + 1 {
            match digits.get(index) {
                None => {
                    result.push(1);
                    break;
                }
                Some(&number) => {
                    if number < 9 {
                        result.push(number + 1);
                        let next = index + 1;
                        result.extend_from_slice(&digits[next..]);
                        break;
                    } else {
                        result.push(0);
                        continue;
                    }
                }
            }
        }

        result.into_iter().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = vec![1, 2, 3];
        let output = Solution::plus_one(input);
        let expected = vec![1, 2, 4];
        assert_eq!(output, expected);
    }

    #[test]
    fn test_case_2() {
        let input = vec![4, 3, 2, 1];
        let output = Solution::plus_one(input);
        let expected = vec![4, 3, 2, 2];
        assert_eq!(output, expected);
    }

    #[test]
    fn test_case_3() {
        let input = vec![9];
        let output = Solution::plus_one(input);
        let expected = vec![1, 0];
        assert_eq!(output, expected);
    }
}
