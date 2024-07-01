/**
 * [211] Design Add and Search Words Data Structure
 */
pub struct Solution {}

// submission codes start here
use std::{rc::Rc, cell::RefCell, collections::{HashMap, VecDeque}};

struct TireNode {
    is_word: bool,
    next: HashMap<char, Rc<RefCell<TireNode>>>,
}

impl TireNode {
    fn new(is_word: bool) -> Self {
        TireNode {
            is_word,
            next: HashMap::new(),
        }
    }
}

struct WordDictionary {
    dummy_head: Rc<RefCell<TireNode>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    fn new() -> Self {
        WordDictionary {
            dummy_head: Rc::new(RefCell::new(TireNode::new(false)))
        }
    }

    fn add_word(&self, word: String) {
        let mut node = Rc::clone(&self.dummy_head);

        for (i, c) in word.chars().enumerate() {
            if node.borrow().next.contains_key(&c) {
                if i == word.len() - 1 {
                    node.borrow().next
                        .get(&c)
                        .unwrap()
                        .borrow_mut().is_word = true;
                }
            } else {
                if i == word.len() - 1 {
                    node.borrow_mut().next.insert(c, Rc::new(RefCell::new(TireNode::new(true))));
                } else {
                    node.borrow_mut().next.insert(c, Rc::new(RefCell::new(TireNode::new(false))));
                }
            }

            let tmp_node = Rc::clone(node.borrow().next.get(&c).unwrap());
            node = tmp_node;
        }
    }

    fn search(&self, word: String) -> bool {
        let mut queue = VecDeque::new();
        queue.push_back(Rc::clone(&self.dummy_head));

        for (i, c) in word.chars().enumerate() {
            if queue.is_empty() {
                break;
            }

            let length = queue.len();

            for _ in 0..length {
                let node = queue.pop_front().unwrap();

                if i == word.len() - 1 {
                    if c == '.' && node.borrow().next.iter().any(|(_, n)| {
                        n.borrow().is_word
                    }) {
                        return true;
                    }

                    if let Some(n) = node.borrow().next.get(&c) {
                        if n.borrow().is_word {
                            return true;
                        }
                    }
                } else {
                    if c == '.' {
                        for n in node.borrow().next.values() {
                            queue.push_back(Rc::clone(n));
                        }
                    } else {
                        if let Some(n) = node.borrow().next.get(&c) {
                            queue.push_back(Rc::clone(n));
                        }
                    }
                }
            }
        }

        false
    }
}

/**
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */

// submission codes end
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_211() {
        let mut dict = WordDictionary::new();

        dict.add_word("bad".to_owned());
        dict.add_word("dad".to_owned());
        dict.add_word("add".to_owned());

        assert!(!dict.search("pad".to_owned()));
        assert!(!dict.search("a".to_owned()));
        assert!(dict.search("dad".to_owned()));
        assert!(dict.search(".ad".to_owned()));
    }
}
