/**
 * [3170] Lexicographically Minimum String After Removing Stars
 */
pub struct Solution {}

// submission codes start here
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
struct CharacterPos {
    char: u8,
    pos: usize,
}

impl Ord for CharacterPos {
    fn cmp(&self, other: &Self) -> Ordering {
        match other.char.cmp(&self.char) {
            Ordering::Equal => self.pos.cmp(&other.pos),
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
        }
    }
}

impl PartialOrd for CharacterPos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn clear_stars(s: String) -> String {
        // Use the binary head to store the minimal and nearest character.
        let mut heap = BinaryHeap::new();
        // The array stores the tuple (character, is_removed).
        let mut buffer: Vec<(u8, bool)> = s.bytes().map(|x| (x, false)).collect();

        for (i, v) in s.bytes().enumerate() {
            if v != b'*' {
                heap.push(CharacterPos { char: v, pos: i });
            } else {
                // Remove the star character firstly.
                buffer.get_mut(i).unwrap().1 = true;

                // Then find the nearest and minimal character.
                let head = heap.pop().unwrap();
                buffer.get_mut(head.pos).unwrap().1 = true;
            }
        }

        String::from_utf8(
            buffer
                .into_iter()
                .filter_map(|(c, f)| if f { None } else { Some(c) })
                .collect(),
        )
        .unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3170() {
        assert_eq!("aab", Solution::clear_stars("aaba*".to_string()));
        assert_eq!("abc", Solution::clear_stars("abc".to_string()));
    }
}
