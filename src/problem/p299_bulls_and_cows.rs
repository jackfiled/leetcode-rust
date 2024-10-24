/**
 * [299] Bulls and Cows
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let secret: Vec<char> = secret.chars().collect();
        let guess: Vec<char> = guess.chars().collect();

        let mut bull = 0;
        let mut cow_map = HashMap::new();
        let mut cow_candidate = Vec::with_capacity(guess.len());

        for i in 0..secret.len() {
            if secret[i] == guess[i] {
                bull += 1;
            } else {
                let entry = cow_map.entry(secret[i]).or_insert(0);
                *entry += 1;

                cow_candidate.push(guess[i]);
            }
        }

        let mut cow = 0;
        for candidate in cow_candidate {
            if let Some(entry) = cow_map.get_mut(&candidate) {
                if *entry != 0 {
                    cow += 1;
                    *entry -= 1;
                }
            }
        }

        format!("{}A{}B", bull, cow)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_299() {
        assert_eq!(
            "1A3B",
            Solution::get_hint("1807".to_owned(), "7810".to_owned())
        );
        assert_eq!(
            "1A1B",
            Solution::get_hint("1123".to_owned(), "0111".to_owned())
        );
    }
}
