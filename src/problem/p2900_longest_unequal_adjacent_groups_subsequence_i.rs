/**
 * [2900] Longest Unequal Adjacent Groups Subsequence I
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let mut result = vec![words[0].clone()];

        let mut last_number = groups[0];

        for i in 1..groups.len() {
            if groups[i] != last_number {
                result.push(words[i].clone());
            }

            last_number = groups[i];
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2900() {
        assert_eq!(
            vec_string!("e", "b"),
            Solution::get_longest_subsequence(vec_string!("e", "a", "b"), vec![0, 0, 1])
        );
        assert_eq!(
            vec_string!("a", "b", "c"),
            Solution::get_longest_subsequence(vec_string!("a", "b", "c", "d"), vec![1, 0, 1, 1])
        );
    }
}
