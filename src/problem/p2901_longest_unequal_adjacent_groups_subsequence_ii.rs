/**
 * [2901] Longest Unequal Adjacent Groups Subsequence II
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn get_words_in_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let n = words.len();
        let mut dp = vec![1; n];
        let mut next_pos = vec![None; n];

        for i in 1..n {
            for j in 0..i {
                if groups[i] == groups[j] {
                    continue;
                }

                if words[i].len() != words[j].len() {
                    continue;
                }

                if Self::calculate_hamming_distance(words[i].as_str(), words[j].as_str()) != 1 {
                    continue;
                }

                if dp[j] + 1 > dp[i] {
                    dp[i] = dp[j] + 1;
                    next_pos[i] = Some(j);
                }
            }
        }

        let max_pos = dp.iter().enumerate().max_by_key(|x| x.1).unwrap();
        let mut pos = max_pos.0;

        let mut result = vec![words[pos].clone()];
        while let Some(p) = next_pos[pos] {
            result.push(words[p].clone());
            pos = p;
        }

        result.into_iter().rev().collect()
    }

    // 计算字符串之间的汉明距
    // 两个字符串需要等长
    fn calculate_hamming_distance(a: &str, b: &str) -> usize {
        a.bytes()
            .zip(b.bytes())
            .filter_map(|(x, y)| if x == y { None } else { Some(()) })
            .count()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2901() {
        assert_eq!(
            vec_string!("dc", "dd", "da"),
            Solution::get_words_in_longest_subsequence(
                vec_string!("bad", "dc", "bc", "ccd", "dd", "da", "cad", "dba", "aba"),
                vec![9, 7, 1, 2, 6, 8, 3, 7, 2]
            )
        );
        assert_eq!(
            vec_string!("bab", "cab"),
            Solution::get_words_in_longest_subsequence(
                vec_string!("bab", "dab", "cab"),
                vec![1, 2, 2]
            )
        );
        assert_eq!(
            vec_string!("a", "b", "c", "d"),
            Solution::get_words_in_longest_subsequence(
                vec_string!("a", "b", "c", "d"),
                vec![1, 2, 3, 4]
            )
        );
    }
}
