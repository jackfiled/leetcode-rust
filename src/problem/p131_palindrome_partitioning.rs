/**
 * [131] Palindrome Partitioning
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let s: Vec<char> = s.chars().collect();
        let length = s.len();

        let mut result = vec![];
        let mut current: Vec<String> = vec![];
        let mut dp = vec![vec![true; length]; length];

        for i in (0..length).rev() {
            for j in i + 1..length {
                dp[i][j] = (s[i] == s[j]) && dp[i + 1][j - 1];
            }
        }

        Self::dfs(&s, 0, &mut dp, &mut current, &mut result);

        result
    }

    fn dfs(
        s: &Vec<char>,
        pos: usize,
        dp: &mut Vec<Vec<bool>>,
        current: &mut Vec<String>,
        result: &mut Vec<Vec<String>>,
    ) {
        if pos >= s.len() {
            result.push(current.clone());
            return;
        }

        for i in pos..s.len() {
            if dp[pos][i] {
                current.push(s[pos..i + 1].iter().collect());
                Self::dfs(s, i + 1, dp, current, result);
                current.pop();
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_131() {
        assert_eq!(
            vec![vec_string!("a", "a", "b"), vec_string!("aa", "b")],
            Solution::partition("aab".to_owned())
        );
    }
}
