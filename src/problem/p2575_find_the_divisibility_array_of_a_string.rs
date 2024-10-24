/**
 * [2575] Find the Divisibility Array of a String
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn divisibility_array(word: String, m: i32) -> Vec<i32> {
        let mut result = vec![0; word.len()];
        let mut now = 0;
        let m = m as i64;

        for (index, c) in word.char_indices() {
            let num = c.to_digit(10).unwrap() as i64;
            now = now * 10 + num;

            if now % m == 0 {
                result[index] = 1;
                now = 0
            } else {
                now = now % m;
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2575() {
        assert_eq!(
            vec![1, 1, 0, 0, 0, 1],
            Solution::divisibility_array("998244".to_owned(), 3)
        );
    }
}
