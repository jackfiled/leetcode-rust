/**
 * [139] Word Break
 */
pub struct Solution {}


// submission codes start here
use std::{rc::Rc, cell::RefCell, collections::HashMap};

struct Trie {
    is_word: bool,
    next: HashMap<char, Rc<RefCell<Trie>>>,
}

impl Trie {
    fn new(is_word: bool) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Trie {
            is_word,
            next: HashMap::new(),
        }))
    }
}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut root = Trie::new(false);

        // 建立字典树
        for word in &word_dict {
            let mut node = Rc::clone(&root);

            for (i, c) in word.chars().enumerate() {
                if node.borrow().next.contains_key(&c) {
                    if i == word.len() - 1 {
                        node.borrow().next.get(&c).unwrap().borrow_mut().is_word = true;
                    }
                } else {
                    if i == word.len() - 1 {
                        node.borrow_mut().next.insert(c, Trie::new(true));
                    } else {
                        node.borrow_mut().next.insert(c, Trie::new(false));
                    }
                }

                let tmp_node = Rc::clone(node.borrow().next.get(&c).unwrap());
                node = tmp_node;
            }
        }

        let word: Vec<char> = s.chars().collect();
        let n = s.len();
        let mut dp = vec![false; n];

        for i in 0..n {
            if i > 0 && !dp[i - 1] {
                continue;
            }

            let mut node = Rc::clone(&root);

            let mut pos = i;
            while pos < n && node.borrow().next.contains_key(&word[pos]) {
                let next_node = Rc::clone(node.borrow().next.get(&word[pos]).unwrap());

                dp[pos] = next_node.borrow().is_word || dp[pos];
                pos += 1;
                node = next_node;
            }
        }

        dp[n - 1]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_139() {
        assert!(Solution::word_break("leetcode".to_owned(), vec_string!("leet", "code")));
        assert!(Solution::word_break("applepenapple".to_owned(), vec_string!("apple", "pen")));
        assert!(!Solution::word_break("catsandog".to_owned(), vec_string!("cats", "cat", "sand", "and", "cat")));
        assert!(Solution::word_break("aaaaaaa".to_owned(), vec_string!("aaa", "aaaa")));
    }
}
