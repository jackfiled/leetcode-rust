/**
 * [2182] Construct String With Repeat Limit
 */
pub struct Solution {}


// submission codes start here
use std::collections::BTreeMap;

impl Solution {
    pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        let mut dict = BTreeMap::new();

        for c in s.chars() {
            let entry = dict.entry(c).or_insert(0);
            *entry += 1;
        }

        let mut result = Vec::new();

        while !dict.is_empty() {
            if let Some((key, mut value)) = dict.pop_last() {
                let mut count = 0;
                while value > 0 {
                    if count < repeat_limit {
                        result.push(key);
                        count += 1;
                        value -= 1;
                    } else {
                        match dict.last_entry() {
                            None => {
                                break;
                            }
                            Some(mut entry) => {
                                result.push(*entry.key());
                                entry.insert(*entry.get() - 1);

                                if entry.get() == &0 {
                                    entry.remove();
                                }

                                count = 0;
                            }
                        }
                    }
                }
            }
        }

        result.iter().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2182() {
        assert_eq!(String::from("zzcccac"), Solution::repeat_limited_string(
            String::from("cczazcc"), 3
        ));
        assert_eq!(String::from("bbabaa"), Solution::repeat_limited_string(
            String::from("aababab"), 2
        ));
    }
}
