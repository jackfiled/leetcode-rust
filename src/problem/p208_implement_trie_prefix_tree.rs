/**
 * [208] Implement Trie (Prefix Tree)
 */
pub struct Solution {}

// submission codes start here
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug)]
struct TrieNode {
    is_ended: bool,
    next: HashMap<char, Rc<RefCell<TrieNode>>>,
}

impl TrieNode {
    fn new(is_ended: bool) -> Self {
        TrieNode {
            is_ended,
            next: HashMap::new(),
        }
    }
}

struct Trie {
    head: Rc<RefCell<TrieNode>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Trie {
            head: Rc::new(RefCell::new(TrieNode::new(false))),
        }
    }

    fn insert(&mut self, word: String) {
        let mut node = Rc::clone(&self.head);

        for (i, c) in word.chars().enumerate() {
            if node.borrow().next.contains_key(&c) {
                if i == word.len() - 1 {
                    node.borrow().next.get(&c).unwrap().borrow_mut().is_ended = true;
                }
            } else {
                if i == word.len() - 1 {
                    node.borrow_mut()
                        .next
                        .insert(c, Rc::new(RefCell::new(TrieNode::new(true))));
                } else {
                    node.borrow_mut()
                        .next
                        .insert(c, Rc::new(RefCell::new(TrieNode::new(false))));
                }
            }

            let tmp_node = Rc::clone(node.borrow().next.get(&c).unwrap());
            node = tmp_node;
        }
    }

    fn search(&self, word: String) -> bool {
        let mut node = Rc::clone(&self.head);

        for (i, c) in word.chars().enumerate() {
            if !node.borrow().next.contains_key(&c) {
                return false;
            }

            if i == word.len() - 1 {
                if !node.borrow().next.get(&c).unwrap().borrow().is_ended {
                    return false;
                }
            }

            let tmp_node = Rc::clone(node.borrow().next.get(&c).unwrap());
            node = tmp_node;
        }

        true
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut node = Rc::clone(&self.head);

        for c in prefix.chars() {
            if !node.borrow().next.contains_key(&c) {
                return false;
            }

            let tmp_node = Rc::clone(node.borrow().next.get(&c).unwrap());
            node = tmp_node;
        }

        true
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_208() {
        let mut trie = Trie::new();

        trie.insert("app".to_owned());

        assert!(!trie.search("ap".to_owned()));

        trie.insert("ap".to_owned());
        assert!(trie.search("ap".to_owned()));
    }
}
