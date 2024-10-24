/**
 * [2834] Find the Minimum Possible Sum of a Beautiful Array
 */
pub struct Solution {}

// submission codes start here
impl Solution {
    pub fn minimum_possible_sum(n: i32, target: i32) -> i32 {
        let n = n as i64;
        let target = target as i64;
        let m = 1e9 as i64 + 7;

        let length = target / 2;

        if length >= n {
            ((1 + n) * n / 2 % m) as i32
        } else {
            (((1 + length) * length / 2 + (target + target + n - length - 1) * (n - length) / 2)
                % m) as i32
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2834() {
        assert_eq!(4, Solution::minimum_possible_sum(2, 3));
        assert_eq!(8, Solution::minimum_possible_sum(3, 3));
        assert_eq!(1, Solution::minimum_possible_sum(1, 1));
    }
}
