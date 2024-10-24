/**
 * [2129] Capitalize the Title
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn capitalize_title(title: String) -> String {
        let mut words: Vec<&str> = title.split(' ').collect();
        let mut result = String::new();

        for word in words.iter_mut() {
            if word.len() < 3 {
                result = result + &word.to_lowercase() + " ";
            } else {
                result = result + &word[0..1].to_uppercase() + &word[1..].to_lowercase() + " ";
            }
        }

        result.trim().to_owned()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2129() {
        assert_eq!(
            "Capitalize The Title",
            Solution::capitalize_title("capiTalIze tHe titLe".to_owned())
        );
        assert_eq!(
            "First Letter of Each Word",
            Solution::capitalize_title("First leTTeR of EACH Word".to_owned())
        );
    }
}
