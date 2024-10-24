/**
 * [331] Verify Preorder Serialization of a Binary Tree
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn is_valid_serialization(preorder: String) -> bool {
        let mut slots = 1;
        let mut i = preorder.chars();

        while let Some(c) = i.next() {
            if slots == 0 {
                return false;
            }

            if (c == ',') {
                continue;
            }

            if (c == '#') {
                slots -= 1;
            } else {
                while let Some(c) = i.next() {
                    if c == ',' {
                        break;
                    }
                }
                slots += 1;
            }
        }

        slots == 0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_331() {
        assert!(Solution::is_valid_serialization(String::from(
            "9,3,4,#,#,1,#,#,2,#,6,#,#"
        )));
        assert!(!Solution::is_valid_serialization(String::from("1,#")));
        assert!(!Solution::is_valid_serialization(String::from("9,#,#,1")));
    }
}
