use std::convert::TryInto;

/**
 * [1969] Minimum Non-Zero Product of the Array Elements
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn min_non_zero_product(p: i32) -> i32 {
        let p = p as i64;
        if p == 1 {
            return 1;
        }        

        let m = 1e9 as i64 + 7;
        let x = Solution::fast_power(2, p, m) - 1;
        let y = 1 << (p - 1);
        dbg!(x, y);

        return (Solution::fast_power(x - 1, y - 1, m) * x % m).try_into().unwrap();
    }

    fn fast_power(x: i64, n: i64, m: i64) -> i64 {
        let mut result = 1;
        let mut n = n;
        let mut x = x;

        while n != 0 {
            if n & 1 != 0 {
                result = result * x % m;
            }

            x = x * x % m;
            n = n >> 1;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1969() {
        assert_eq!(1, Solution::min_non_zero_product(1));
        assert_eq!(6, Solution::min_non_zero_product(2));
    }
}
