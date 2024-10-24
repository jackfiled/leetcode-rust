/**
 * [76] Minimum Window Substring
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut t_map = HashMap::new();

        for c in t.chars() {
            let entry = t_map.entry(c).or_insert(0);
            *entry += 1;
        }

        let s: Vec<char> = s.chars().collect();
        let mut s_map = HashMap::new();

        let (mut i, mut j) = (0, 0);
        let mut result_left = 0;
        let mut result_length = usize::MAX;

        while j < s.len() {
            if t_map.contains_key(&s[j]) {
                let entry = s_map.entry(s[j]).or_insert(0);
                *entry += 1;
            }

            while Solution::check_map(&s_map, &t_map) && i <= j {
                if j - i + 1 < result_length {
                    result_length = j - i + 1;
                    result_left = i;
                }

                if let Some(entry) = s_map.get_mut(&s[i]) {
                    *entry -= 1;
                }
                i += 1;
            }

            j += 1;
        }

        if result_length == usize::MAX {
            String::new()
        } else {
            s[result_left..result_left + result_length].iter().collect()
        }
    }

    fn check_map(s: &HashMap<char, i32>, t: &HashMap<char, i32>) -> bool {
        for (key, value) in t {
            match s.get(key) {
                Some(real_value) => {
                    if *value > *real_value {
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

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_76() {
        assert_eq!(
            "BANC".to_owned(),
            Solution::min_window("ADOBECODEBANC".to_owned(), "ABC".to_owned())
        );
    }
}
