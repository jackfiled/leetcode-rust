/**
 * [383] Ransom Note
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut map = HashMap::new();

        for c in magazine.chars() {
            let entry = map.entry(c).or_insert(0);

            *entry += 1;
        }

        for c in ransom_note.chars() {
            match map.get_mut(&c) {
                Some(entry) => {
                    if *entry == 0 {
                        return false;
                    }

                    *entry -= 1;
                }
                None => {
                    return false;
                }
            }
        }

        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_383() {}
}
