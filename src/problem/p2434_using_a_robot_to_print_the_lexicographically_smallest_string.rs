/**
 * [2434] Using a Robot to Print the Lexicographically Smallest String
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn robot_with_string(s: String) -> String {
        let s: Vec<u8> = s.bytes().collect();
        let mut character_map = HashMap::new();

        for &c in s.iter() {
            let entry = character_map.entry(c).or_insert(0);
            *entry += 1;
        }

        // The Minimal value in the s.
        let mut min_character = b'a';
        let mut result = Vec::with_capacity(s.len());
        let mut buffer = Vec::with_capacity(s.len());

        for &c in s.iter() {
            buffer.push(c);
            *character_map.get_mut(&c).unwrap() -= 1;

            // Find the minimal value in the s.
            while min_character != b'z' && character_map.get(&min_character).is_none_or(|x| *x == 0)
            {
                min_character += 1;
            }

            while let Some(&head) = buffer.last() {
                if head <= min_character {
                    result.push(head);
                    buffer.pop();
                } else {
                    break;
                }
            }
        }

        String::from_utf8(result).unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2434() {
        assert_eq!("azz", Solution::robot_with_string("zza".to_string()));
        assert_eq!("abc", Solution::robot_with_string("bac".to_string()));
    }
}
