/**
 * [3222] Find the Winning Player in Coin Game
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn losing_player(x: i32, y: i32) -> String {
        let count = x.min(y / 4);

        if count % 2 == 1 {
            "Alice".to_owned()
        } else {
            "Bob".to_owned()
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3222() {
        assert_eq!("Alice".to_owned(), Solution::losing_player(2, 7));
        assert_eq!("Bob".to_owned(), Solution::losing_player(4, 11));
    }
}
