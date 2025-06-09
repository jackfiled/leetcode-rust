/**
 * [440] K-th Smallest in Lexicographical Order
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn find_kth_number(n: i32, mut k: i32) -> i32 {
        let mut currrent = 1;
        let n = n as i64;
        k -= 1;

        while k > 0 {
            let steps = Self::calculate_steps(currrent, n);
            // If steps <= k, then the number is in the tree of next current (at least).
            if steps <= k {
                k -= steps;
                currrent += 1;
            } else {
                // The number is in the current tree.
                currrent = currrent * 10;
                k -= 1;
            }
        }

        currrent as i32
    }

    /// Calculate the count of number with prefix current.
    fn calculate_steps(current: i64, n: i64) -> i32 {
        let mut result = 0;
        let (mut first, mut last) = (current, current);

        while first <= n {
            result += n.min(last) - first + 1;
            first = first * 10;
            last = last * 10 + 9;
        }

        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_440() {
        assert_eq!(10, Solution::find_kth_number(13, 2));
        assert_eq!(1, Solution::find_kth_number(1, 1));
    }
}
