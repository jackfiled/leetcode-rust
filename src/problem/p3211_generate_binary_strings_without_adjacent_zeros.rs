/**
 * [3211] Generate Binary Strings Without Adjacent Zeros
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn valid_strings(n: i32) -> Vec<String> {
        let n = n as usize;
        let mut result = Vec::new();
        let mut str = vec!['0'; n];

        Self::dfs(0, false, &mut str, &mut result);

        result
    }

    fn dfs(i: usize, last_zero: bool, str: &mut Vec<char>, result: &mut Vec<String>) {
        if i >= str.len() {
            result.push(str.iter().collect());
            return;
        }

        if !last_zero {
            str[i] = '0';
            Self::dfs(i + 1, true, str, result);
        }

        str[i] = '1';
        Self::dfs(i + 1, false, str, result);
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3211() {
        assert_eq!(
            vec_string!("010", "011", "101", "110", "111"),
            Solution::valid_strings(3)
        );
        assert_eq!(vec_string!("0", "1"), Solution::valid_strings(1));
    }
}
