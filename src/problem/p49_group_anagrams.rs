/**
 * [49] Group Anagrams
 */
pub struct Solution {}

// submission codes start here
use std::{collections::HashMap, hash::Hash};

#[derive(Debug)]
struct WordState {
    characters: HashMap<char, usize>,
}

impl PartialEq for WordState {
    fn eq(&self, other: &Self) -> bool {
        if self.characters.len() != other.characters.len() {
            return false;
        }

        for (c, &v) in self.characters.iter() {
            match other.characters.get(c) {
                Some(other_v) => {
                    if *other_v != v {
                        return false;
                    }
                }
                None => {
                    return false;
                }
            }
        }

        true
    }
}

impl Eq for WordState {}

impl Hash for WordState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let mut chars: Vec<(&char, &usize)> = 
            self.characters.iter().map(|p| p).collect();
        
        chars.sort_by_key(|p| p.0);

        for (&c, &v) in chars {
            c.hash(state);
            v.hash(state);   
        }
    }
}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut result = HashMap::with_capacity(strs.len());

        for word in strs {
            let mut characters = HashMap::with_capacity(26);

            for c in word.chars() {
                let entry = characters.entry(c).or_insert(0);
                *entry += 1;
            }

            let state = WordState { characters };
            dbg!(&state);
            let entry = result.entry(state).or_insert(vec![]);
            entry.push(word);
        }

        result.into_values().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_49() {
        assert_eq!(
            WordState {
                characters: HashMap::from([('e', 1), ('a', 1), ('t', 1)])
            },
            WordState {
                characters: HashMap::from([('a', 1), ('t', 1), ('e', 1)])
            }
        );
    }
}
