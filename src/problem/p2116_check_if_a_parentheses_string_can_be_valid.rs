/**
 * [2116] Check if a Parentheses String Can Be Valid
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn can_be_valid(s: String, locked: String) -> bool {
        let s: Vec<u8> = s.bytes().collect();
        let locked: Vec<bool> = locked
            .bytes()
            .map(|x| if x == b'0' { false } else { true })
            .collect();

        if s.len() % 2 != 0 {
            return false;
        }

        let mut locked_left = vec![];
        let mut unlocked_pos = vec![];

        for (i, &v) in s.iter().enumerate() {
            if !locked[i] {
                unlocked_pos.push(i);
            } else {
                if v == b'(' {
                    locked_left.push(i);
                } else {
                    // 首先使用锁定的左括号匹配
                    if locked_left.pop().is_none() {
                        // 然后使用未锁定的位置修改
                        if unlocked_pos.pop().is_none() {
                            return false;
                        }
                    }
                }
            }
        }

        // 判断剩余的未锁定位置和锁定的左括号是否匹配
        while let Some(left_pos) = locked_left.pop() {
            if let Some(unlock_pos) = unlocked_pos.pop() {
                if left_pos > unlock_pos {
                    return false;
                }
            } else {
                return false;
            }
        }

        // 剩余的未锁定括号必须是偶数个
        unlocked_pos.len() % 2 == 0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2116() {
        assert!(!Solution::can_be_valid(
            "(((())".to_owned(),
            "111111".to_owned()
        ));
        assert!(Solution::can_be_valid(
            "))()))".to_owned(),
            "010100".to_owned()
        ));
        assert!(Solution::can_be_valid("()()".to_owned(), "0000".to_owned()));
        assert!(!Solution::can_be_valid(")".to_owned(), "0".to_owned()));
        assert!(Solution::can_be_valid(
            "(((())(((())".to_owned(),
            "111111010111".to_owned()
        ));
    }
}
