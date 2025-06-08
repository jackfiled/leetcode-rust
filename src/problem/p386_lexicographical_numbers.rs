/**
 * [386] Lexicographical Numbers
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity(n as usize);

        for i in 1..10 {
            Self::search(&mut result, i, n);
        }

        result
    }

    fn search(result: &mut Vec<i32>, mut prefix: i32, n: i32) {
        if prefix <= n {
            result.push(prefix);
        }

        prefix = prefix * 10;

        for i in 0..10 {
            if prefix + i > n {
                break;
            }

            Self::search(result, prefix + i, n);
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_386() {
        assert_eq!(vec![1, 2], Solution::lexical_order(2));
        assert_eq!(
            vec![1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9],
            Solution::lexical_order(13)
        );
    }
}
