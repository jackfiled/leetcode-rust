/**
 * [224] Basic Calculator
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut operator_stack = vec![1];
        let mut result: i64 = 0;
        let mut sign = 1;

        let s: Vec<char> = s.chars().collect();

        let mut i = 0;

        while i < s.len() {
            if s[i] == ' ' {
                i += 1;
            } else if s[i] == '+' {
                sign = *operator_stack.last().unwrap();
                i += 1;
            } else if s[i] == '-' {
                sign = -(*operator_stack.last().unwrap());
                i += 1;
            } else if s[i] == '(' {
                operator_stack.push(sign);
                i += 1;
            } else if s[i] == ')' {
                operator_stack.pop();
                i += 1;
            } else {
                let mut num: i64 = 0;

                while i < s.len() && s[i].is_numeric() {
                    num = num * 10 + s[i].to_digit(10).unwrap() as i64;
                    i += 1;
                }

                result += sign * num;
            }
        }

        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_224() {
        assert_eq!(2, Solution::calculate("1 + 1".to_owned()));
        assert_eq!(3, Solution::calculate(" 2-1 + 2 ".to_owned()));
        assert_eq!(23, Solution::calculate("(1+(4+5+2)-3)+(6+8)".to_owned()));
        assert_eq!(2147483647, Solution::calculate("2147483647".to_owned()));
    }
}
