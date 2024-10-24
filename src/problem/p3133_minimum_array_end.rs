/**
 * [3133] Minimum Array End
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn min_end(n: i32, x: i32) -> i64 {
        let (n, x) = (n as i64, x as i64);
        let mut result = x;

        let bit_count = 128 - n.leading_zeros() - x.leading_zeros();

        let mut pos = 0;

        for i in 0..bit_count {
            if (result >> i) & 1 == 0 {
                if ((n - 1) >> pos) & 1 == 1 {
                    result |= 1 << i;
                }

                pos += 1;
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3133() {
        assert_eq!(6, Solution::min_end(3, 4));
        assert_eq!(15, Solution::min_end(2, 7));
    }
}
