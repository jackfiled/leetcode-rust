/**
 * [3226] Number of Bit Changes to Make Two Integers Equal
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn min_changes(n: i32, k: i32) -> i32 {
        if n == k {
            return 0;
        }

        let (mut n, mut k) = (n, k);
        let mut result = 0;

        while n != 0 {
            if n % 2 == 0 {
                if k % 2 != 0 {
                    result = -1;
                    break;
                }
            } else {
                if k % 2 != 1 {
                    result += 1;
                }
            }

            k = k / 2;
            n = n / 2;
        }

        if k != 0 {
            -1
        } else {
            result
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3226() {
        assert_eq!(2, Solution::min_changes(13, 4));
        assert_eq!(0, Solution::min_changes(21, 21));
        assert_eq!(-1, Solution::min_changes(14, 13));
        assert_eq!(-1, Solution::min_changes(11, 56));
    }
}
