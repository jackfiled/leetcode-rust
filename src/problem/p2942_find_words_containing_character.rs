/**
 * [2942] Find Words Containing Character
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
        words
            .iter()
            .enumerate()
            .filter_map(|(i, v)| if v.contains(x) { Some(i as i32) } else { None })
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2942() {
        assert_eq!(
            vec![0, 1],
            Solution::find_words_containing(vec_string!("leet", "code"), 'e')
        );
        assert_eq!(
            vec![0, 2],
            Solution::find_words_containing(vec_string!("abc", "bcd", "aaaa", "cbc"), 'a')
        );
        assert_eq!(
            Vec::<i32>::new(),
            Solution::find_words_containing(vec_string!("abc", "bcd", "aaaa", "cbc"), 'z'),
        );
    }
}
