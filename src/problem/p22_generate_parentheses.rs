/**
 * [22] Generate Parentheses
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let n = n as usize;
        let mut result = vec![];
        let mut path = Vec::with_capacity(n * 2);

        Self::backtrace(n, 0, &mut path, &mut result);

        result
    }

    fn backtrace(left: usize, right: usize, path: &mut Vec<char>, result: &mut Vec<String>) {
        if left == 0 && right == 0 {
            let t = path.clone();
            result.push(t.iter().collect());
            return;
        }

        if left != 0 {
            path.push('(');
            Self::backtrace(left - 1, right + 1, path, result);
            path.pop();
        }

        if right != 0 {
            path.push(')');
            Self::backtrace(left, right - 1, path, result);
            path.pop();
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_22() {}
}
