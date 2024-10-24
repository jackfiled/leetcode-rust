pub struct Solution;

impl Solution {
    pub fn report_spam(message: Vec<String>, banned_words: Vec<String>) -> bool {
        use std::collections::HashSet;

        let mut set = HashSet::with_capacity(banned_words.len());
        for i in banned_words {
            set.insert(i);
        }

        let mut result = 0;

        for i in message.iter() {
            if set.contains(i) {
                result += 1;
            }

            if result >= 2 {
                return true;
            }
        }

        false
    }
}
