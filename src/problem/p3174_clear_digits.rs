/**
 * [3174] Clear Digits
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn clear_digits(s: String) -> String {
        let s: Vec<char> = s.chars().collect();
        let mut mark = vec![true; s.len()];
        
        for (i, &c) in s.iter().enumerate() {
            if !c.is_ascii_digit() {
                continue;
            }
            
            // 是数字
            mark[i] = false;
            let mut last = i - 1;

            while !mark[last] {
                last -= 1;
            }
            mark[last] = false;
        }
        
        s.iter().enumerate().filter_map(|(i, &c)| {
            if mark[i] {
                Some(c)
            } else { 
                None
            }
        }).collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3174() {
        assert_eq!("abc".to_owned(), Solution::clear_digits("abc".to_owned()));
        assert_eq!("".to_owned(), Solution::clear_digits("cb34".to_owned()));
    }
}
