/**
 * [541] Reverse String II
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let s: Vec<char> = s.chars().collect();

        let mut reverse = true;
        let k = k as usize;

        let mut result = Vec::with_capacity(s.len());
        let mut buffer = Vec::with_capacity(k);
        let mut pos = 0;

        'outer: loop {
            for _ in 0..k {
                if pos == s.len() {
                    while let Some(c) = buffer.pop() {
                        result.push(c);
                    }

                    break 'outer;
                }

                if reverse {
                    buffer.push(s[pos]);
                } else {
                    result.push(s[pos]);
                }
                pos += 1;
            }

            while let Some(c) = buffer.pop() {
                result.push(c);
            }
            reverse = !reverse;
        }

        result.into_iter().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_541() {
        assert_eq!(
            "bacdfeg".to_owned(),
            Solution::reverse_str("abcdefg".to_owned(), 2)
        );
        assert_eq!(
            "bacd".to_owned(),
            Solution::reverse_str("abcd".to_owned(), 2)
        );
    }
}
