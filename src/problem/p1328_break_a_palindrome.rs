/**
 * [1328] Break a Palindrome
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn break_palindrome(palindrome: String) -> String {
        if palindrome.len() == 1 {
            return "".to_owned();
        }

        let mut s: Vec<char> = palindrome.chars().collect();

        let mut flag = false;
        for i in 0..s.len() / 2 {
            if s[i] != 'a' {
                s[i] = 'a';
                flag = true;
                break;
            }
        }

        // 说明s的前半段全部都是a
        // 直接把最后一个字符改成b
        if !flag {
            let pos = s.len() - 1;
            s[pos] = 'b';
        }

        s.into_iter().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1328() {
        assert_eq!("aabab", Solution::break_palindrome("aabaa".to_owned()));
        assert_eq!("ab", Solution::break_palindrome("aa".to_owned()));
        assert_eq!("ab", Solution::break_palindrome("bb".to_owned()));
        assert_eq!("aaccba", Solution::break_palindrome("abccba".to_owned()));
        assert_eq!("", Solution::break_palindrome("a".to_owned()));
    }
}
