/**
 * [150] Evaluate Reverse Polish Notation
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];

        for token in tokens {
            if let Ok(num) = token.parse::<i32>() {
                stack.push(num);
            } else {
                let second = stack.pop().unwrap();
                let first = stack.pop().unwrap();

                match token.as_str() {
                    "+" => stack.push(first + second),
                    "-" => stack.push(first - second),
                    "*" => stack.push(first * second),
                    "/" => stack.push(first / second),
                    _ => {},
                }
            }
        }

        stack[0]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_150() {
        assert_eq!(9, Solution::eval_rpn(vec_string!["2","1","+","3","*"]));
    }
}
