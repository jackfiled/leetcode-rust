/**
 * [3291] Minimum Number of Valid Strings to Form Target I
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn min_valid_strings(words: Vec<String>, target: String) -> i32 {
        let words: Vec<Vec<char>> = words.into_iter().map(|x| x.chars().collect()).collect();
        let target: Vec<char> = target.chars().collect();

        let calculate_prefix = |word: &Vec<char>, target: &Vec<char>| {
            let s: Vec<char> = word
                .iter()
                .chain(['#'].iter())
                .chain(target.iter())
                .map(|x| x.clone())
                .collect();
            let mut result = vec![0; s.len()];

            for i in 1..s.len() {
                let mut j = result[i - 1];
                while j > 0 && s[i] != s[j] {
                    j = result[j - 1];
                }

                if s[i] == s[j] {
                    j += 1;
                }
                result[i] = j;
            }

            result
        };

        let n = target.len();
        let mut back = vec![0; n];
        for word in words.iter() {
            let pi = calculate_prefix(word, &target);
            let m = word.len();

            for i in 0..n {
                back[i] = back[i].max(pi[m + 1 + i]);
            }
        }

        let mut dp = vec![0; n + 1];

        for i in 1..=n {
            dp[i] = 1e9 as i32;
        }

        for i in 0..n {
            dp[i + 1] = dp[i + 1 - back[i]] + 1;
            if dp[i + 1] > n as i32 {
                return -1;
            }
        }

        dp[n]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3291() {
        assert_eq!(
            3,
            Solution::min_valid_strings(
                vec_string!("abc", "aaaaa", "bcdef"),
                "aabcdabc".to_owned()
            )
        );
        assert_eq!(
            2,
            Solution::min_valid_strings(vec_string!("abababab", "ab"), "ababaababa".to_owned())
        );
        assert_eq!(
            -1,
            Solution::min_valid_strings(vec_string!("abcdef"), "xyz".to_owned())
        );
    }
}
