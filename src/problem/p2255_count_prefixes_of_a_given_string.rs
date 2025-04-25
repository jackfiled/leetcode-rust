/**
 * [2255] Count Prefixes of a Given String
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn count_prefixes(words: Vec<String>, s: String) -> i32 {
        words.iter().filter(|i| s.starts_with(i.as_str())).count() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2255() {
        assert_eq!(
            3,
            Solution::count_prefixes(
                vec_string!("a", "b", "c", "ab", "bc", "abc"),
                "abc".to_owned()
            )
        );
        assert_eq!(
            2,
            Solution::count_prefixes(vec_string!("a", "a"), "aa".to_owned())
        );
    }
}
