/**
 * [212] Word Search II
 */
pub struct Solution {}

// submission codes start here
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

#[derive(Debug)]
struct TrieNode {
    is_word: bool,
    index: usize,
    next: HashMap<char, Rc<RefCell<TrieNode>>>,
}

impl TrieNode {
    fn new(is_word: bool, index: usize) -> TrieNode {
        TrieNode {
            is_word,
            index,
            next: HashMap::new(),
        }
    }
}

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let dummy_head = Rc::new(RefCell::new(TrieNode::new(false, usize::MAX)));

        // 创建字典树
        for (index, word) in words.iter().enumerate() {
            let mut node = Rc::clone(&dummy_head);
            for (i, c) in word.chars().enumerate() {
                if node.borrow().next.contains_key(&c) {
                    if i == word.len() - 1 {
                        node.borrow().next.get(&c).unwrap().borrow_mut().is_word = true;
                        node.borrow().next.get(&c).unwrap().borrow_mut().index = index;
                    }
                } else {
                    if i == word.len() - 1 {
                        node.borrow_mut()
                            .next
                            .insert(c, Rc::new(RefCell::new(TrieNode::new(true, index))));
                    } else {
                        node.borrow_mut()
                            .next
                            .insert(c, Rc::new(RefCell::new(TrieNode::new(false, usize::MAX))));
                    }
                };

                let tmp_node = Rc::clone(node.borrow().next.get(&c).unwrap());
                node = tmp_node;
            }
        }

        let mut result = HashSet::new();

        let (m, n) = (board.len(), board[0].len());
        for i in 0..(m as i32) {
            for j in 0..(n as i32) {
                let mut visited = vec![vec![false; n]; m];
                Self::dfs(&board, &dummy_head, &words, &mut result, &mut visited, i, j);
            }
        }

        result.iter().map(|s| s.to_owned()).collect()
    }

    fn dfs(
        board: &Vec<Vec<char>>,
        node: &Rc<RefCell<TrieNode>>,
        words: &Vec<String>,
        result: &mut HashSet<String>,
        visited: &mut Vec<Vec<bool>>,
        x: i32,
        y: i32,
    ) {
        let (m, n) = (board.len() as i32, board[0].len() as i32);

        if x < 0 || x >= m || y < 0 || y >= n {
            return;
        }

        if visited[x as usize][y as usize] {
            return;
        }
        visited[x as usize][y as usize] = true;

        let c = board[x as usize][y as usize];

        if let Some(next) = node.borrow().next.get(&c) {
            if next.borrow().is_word {
                result.insert(words[next.borrow().index].clone());
            }

            Self::dfs(board, next, words, result, visited, x, y + 1);
            Self::dfs(board, next, words, result, visited, x - 1, y);
            Self::dfs(board, next, words, result, visited, x + 1, y);
            Self::dfs(board, next, words, result, visited, x, y - 1);
        };

        visited[x as usize][y as usize] = false;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_212_1() {
        let board = vec![
            vec!['o', 'a', 'a', 'n'],
            vec!['e', 't', 'a', 'e'],
            vec!['i', 'h', 'k', 'r'],
            vec!['i', 'f', 'l', 'v'],
        ];

        let words = vec_string!("oath", "pea", "eat", "rain");

        let result = Solution::find_words(board, words);

        dbg!(&result);
        assert_eq!(result.len(), 2);
        assert!(result.contains(&"eat".to_owned()));
        assert!(result.contains(&"oath".to_owned()));
    }

    #[test]
    fn test_212_2() {
        let board = vec![vec!['a', 'a']];

        let words = vec_string!("aaa");

        let result = Solution::find_words(board, words);

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_212_3() {
        let board = vec![
            vec!['a', 'b', 'c', 'e'],
            vec!['x', 'x', 'c', 'd'],
            vec!['x', 'x', 'b', 'a'],
        ];

        let words = vec_string!("abc", "abcd");

        let result = Solution::find_words(board, words);

        assert_eq!(result.len(), 2);
        assert!(result.contains(&"abc".to_owned()));
        assert!(result.contains(&"abcd".to_owned()));
    }
}
