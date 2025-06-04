/**
 * [3403] Find the Lexicographically Largest String From the Box I
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn answer_string(word: String, num_friends: i32) -> String {
        // When there is only one person, the word can not be split.
        if num_friends == 1 {
            return word;
        }

        let word: Vec<u8> = word.bytes().collect();
        let length = word.len() - num_friends as usize + 1;
        let mut result: Option<&[u8]> = None;

        for i in 0..word.len() {
            let end = (i + length).min(word.len());
            let w = &word[i..(i + length).min(word.len())];

            if let Some(r) = result {
                if w > r {
                    result = Some(w)
                }
            } else {
                result = Some(w);
            }
        }

        String::from_utf8(result.unwrap().into_iter().map(|x| *x).collect()).unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3403() {
        assert_eq!("gh", Solution::answer_string("gh".to_string(), 1));
        assert_eq!("dbc", Solution::answer_string("dbca".to_string(), 2));
        assert_eq!("g", Solution::answer_string("gggg".to_string(), 4));
    }
}
