/**
 * [2207] Maximize Number of Subsequences in a String
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn maximum_subsequence_count(text: String, pattern: String) -> i64 {
        let pattern: Vec<char> = pattern.chars().collect();
        let (first, second) = (pattern[0], pattern[1]);

        let (mut first_count, mut second_count) = (0, 0);
        let mut result = 0;

        for c in text.chars() {
            if c == second {
                result += first_count;
                second_count += 1;
            }

            if c == first {
                first_count += 1;
            }
        }

        result + first_count.max(second_count)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2207() {
        assert_eq!(
            4,
            Solution::maximum_subsequence_count("abdcdbc".to_owned(), "ac".to_owned())
        );
        assert_eq!(
            6,
            Solution::maximum_subsequence_count("aabb".to_owned(), "ab".to_owned())
        );
    }
}
