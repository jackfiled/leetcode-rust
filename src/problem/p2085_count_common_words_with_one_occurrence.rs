/**
 * [2085] Count Common Words With One Occurrence
 *
 * Given two string arrays words1 and words2, return the number of strings that appear exactly once in each of the two arrays.
 *  
 * <strong class="example">Example 1:
 *
 * Input: words1 = ["leetcode","is","amazing","as","is"], words2 = ["amazing","leetcode","is"]
 * Output: 2
 * Explanation:
 * - "leetcode" appears exactly once in each of the two arrays. We count this string.
 * - "amazing" appears exactly once in each of the two arrays. We count this string.
 * - "is" appears in each of the two arrays, but there are 2 occurrences of it in words1. We do not count this string.
 * - "as" appears once in words1, but does not appear in words2. We do not count this string.
 * Thus, there are 2 strings that appear exactly once in each of the two arrays.
 *
 * <strong class="example">Example 2:
 *
 * Input: words1 = ["b","bb","bbb"], words2 = ["a","aa","aaa"]
 * Output: 0
 * Explanation: There are no strings that appear in each of the two arrays.
 *
 * <strong class="example">Example 3:
 *
 * Input: words1 = ["a","ab"], words2 = ["a","a","a","ab"]
 * Output: 1
 * Explanation: The only string that appears exactly once in each of the two arrays is "ab".
 *
 *  
 * Constraints:
 *
 * 	1 <= words1.length, words2.length <= 1000
 * 	1 <= words1[i].length, words2[j].length <= 30
 * 	words1[i] and words2[j] consists only of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/count-common-words-with-one-occurrence/
// discuss: https://leetcode.cn/problems/count-common-words-with-one-occurrence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashMap;

impl Solution {
    pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
        let mut dict = HashMap::new();

        for i in &words1 {
            match dict.get(i) {
                None => {
                    dict.insert(i, 0);
                }
                Some(_) => {
                    dict.insert(i, 1);
                }
            }
        }

        for i in &words2 {
            match dict.get(i) {
                None => {}
                Some(value) => {
                    if *value == 0 {
                        dict.insert(i, 2);
                    } else if *value == 2 {
                        dict.insert(i, 1);
                    }
                }
            }
        }

        dict.values().filter(|x| **x == 2).count() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2085() {}
}
