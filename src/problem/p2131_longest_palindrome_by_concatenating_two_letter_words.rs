/**
 * [2131] Longest Palindrome by Concatenating Two Letter Words
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut counter = HashMap::new();
        let mut same_counter = HashMap::new();

        for word in words.iter() {
            let mut chars = word.bytes();
            let first = (chars.next().unwrap() - b'a') as i32;
            let second = (chars.next().unwrap() - b'a') as i32;

            if first < second {
                let hash = first * 26 + second;
                let entry = counter.entry(hash).or_insert((0, 0));
                entry.0 += 1;
            } else if first > second {
                let hash = second * 26 + first;
                let entry = counter.entry(hash).or_insert((0, 0));
                entry.1 += 1;
            } else {
                let entry = same_counter.entry(first).or_insert(0);
                *entry += 1;
            }
        }

        // 对于两个不同的字符
        // 取计数的最小值
        let mut result: i32 = counter
            .values()
            .filter_map(|&(x, y)| Some(x.min(y) * 4))
            .sum();

        // 对于两个相同的字符
        // 如果成对就可以加入答案
        result += same_counter
            .values()
            .filter_map(|&v| Some(v / 2 * 4))
            .sum::<i32>();

        // 再判断是否有落单的相同字符放在中间
        // 注意只能放一次
        result += same_counter
            .values()
            .filter_map(|&v| if v % 2 != 0 { Some(()) } else { None })
            .next()
            .map_or(0, |()| 2);

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2131() {
        assert_eq!(
            6,
            Solution::longest_palindrome(vec_string!("aa", "aa", "aa"))
        );
        assert_eq!(
            6,
            Solution::longest_palindrome(vec_string!["lc", "cl", "gg"])
        );
        assert_eq!(
            8,
            Solution::longest_palindrome(vec_string!["ab", "ty", "yt", "lc", "cl", "ab"])
        );
        assert_eq!(
            2,
            Solution::longest_palindrome(vec_string!["ll", "cc", "xx"])
        );
    }
}
