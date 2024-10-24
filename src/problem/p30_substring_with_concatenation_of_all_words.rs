/**
 * [30] Substring with Concatenation of All Words
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut result = Vec::new();

        let m = words.len();
        let n = words[0].len();
        let ls = s.len();

        for start in 0..n {
            if start + m * n > ls {
                break;
            }

            let mut differ = HashMap::new();

            for i in 0..m {
                let word = s[start + i * n..start + i * n + n].to_owned();

                let entry = differ.entry(word).or_insert(0);
                *entry += 1;
            }

            for word in (&words).to_owned() {
                let entry = differ.entry(word.clone()).or_insert(0);
                *entry -= 1;

                if *entry == 0 {
                    differ.remove(&word);
                }
            }

            // 滑动窗口开始滑动
            for i in (start..ls - m * n + 1).step_by(n) {
                if i != start {
                    // 新加入的word
                    let end_word = s[i + (m - 1) * n..i + m * n].to_owned();
                    let entry = differ.entry(end_word.clone()).or_insert(0);
                    *entry += 1;

                    if *entry == 0 {
                        differ.remove(&end_word);
                    }

                    // 滑动后被移除的word
                    let begin_word = s[i - n..i].to_owned();
                    let entry = differ.entry(begin_word.clone()).or_insert(0);
                    *entry -= 1;

                    if *entry == 0 {
                        differ.remove(&begin_word);
                    }
                }

                if differ.len() == 0 {
                    result.push(i as i32);
                }
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_30() {
        assert_eq!(
            vec![0, 9],
            Solution::find_substring(
                "barfoothefoobarman".to_owned(),
                vec!["foo".to_owned(), "bar".to_owned()]
            )
        )
    }
}
