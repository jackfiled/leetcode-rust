/**
 * [127] Word Ladder
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashSet;

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut start = usize::MAX;
        let mut start_found = false;
        let mut end_found = false;
        let mut end = usize::MAX;

        for (i, word) in word_list.iter().enumerate() {
            if *word == begin_word {
                start_found = true;
                start = i;
                break;
            }
        }

        let mut neighbors = vec![HashSet::new(); word_list.len()];

        // 构建邻接矩阵
        for i in 0..word_list.len() {
            for j in 1..word_list.len() {
                if Self::is_one_char(&word_list[i], &word_list[j]) {
                    neighbors[i].insert(j);
                    neighbors[j].insert(i);
                }
            }

            if word_list[i] == end_word {
                end_found = true;
                end = i;
            }

            if !start_found {
                if Self::is_one_char(&word_list[i], &begin_word) {
                    start = i;
                }
            }
        }

        if !end_found {
            return 0;
        }

        // 如果开始和结束相同
        // 则必然是跳了一步
        if start == end {
            return 2;
        }

        let mut result = i32::MAX;

        // dijkstra算法
        let mut distances = vec![i32::MAX; word_list.len()];
        let mut flags = vec![false; word_list.len()];
        distances[start] = 0;
        flags[start] = true;
        for &i in &neighbors[start] {
            distances[i] = 1;
        }

        for _ in 1..word_list.len() {
            let mut min = i32::MAX;
            let mut next = 0;

            for i in 0..word_list.len() {
                if !flags[i] && distances[i] < min {
                    min = distances[i];
                    next = i;
                }
            }

            flags[next] = true;
            if next == end {
                result = result.min(min);
                break;
            }

            for &i in &neighbors[next] {
                if !flags[i] && min + 1 < distances[i] {
                    distances[i] = min + 1;
                }
            }
        }

        if result == i32::MAX {
            0
        } else {
            if start_found {
                result + 1
            } else {
                result + 2
            }
        }
    }

    fn is_one_char(a: &String, b: &String) -> bool {
        let mut one_char = false;

        for (c1, c2) in a.chars().zip(b.chars()) {
            if c1 != c2 {
                if one_char {
                    return false;
                } else {
                    one_char = true
                }
            }
        }

        one_char
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_127() {
        assert_eq!(
            Solution::ladder_length(
                "hit".to_owned(),
                "cog".to_owned(),
                vec![
                    "hot".to_owned(),
                    "dot".to_owned(),
                    "dog".to_owned(),
                    "lot".to_owned(),
                    "log".to_owned(),
                    "cog".to_owned()
                ]
            ),
            5
        );

        assert_eq!(
            Solution::ladder_length(
                "a".to_owned(),
                "c".to_owned(),
                vec!["a".to_owned(), "b".to_owned(), "c".to_owned()]
            ),
            2
        );

        assert_eq!(
            Solution::ladder_length("a".to_owned(), "c".to_owned(), vec!["c".to_owned()]),
            2
        );

        assert_eq!(
            Solution::ladder_length(
                "hot".to_owned(),
                "dog".to_owned(),
                vec!["hot".to_owned(), "dot".to_owned(), "dog".to_owned(),]
            ),
            3
        );

        assert_eq!(
            Solution::ladder_length(
                "lost".to_owned(),
                "cost".to_owned(),
                vec![
                    "most".to_owned(),
                    "fist".to_owned(),
                    "lost".to_owned(),
                    "cost".to_owned(),
                    "fish".to_owned()
                ]
            ),
            2
        );
    }
}
