/**
 * [2595] Number of Even and Odd Bits
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn even_odd_bit(mut n: i32) -> Vec<i32> {
        let mut is_even = true;
        let mut even_count = 0;
        let mut odd_count = 0;

        while n > 0 {
            let bit = n % 2;
            if bit == 1 {
                if is_even {
                    even_count += 1;
                } else {
                    odd_count += 1;
                }
            }

            is_even = !is_even;
            n = n / 2;
        }

        vec![even_count, odd_count]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2595() {
        assert_eq!(vec![1, 2], Solution::even_odd_bit(50));
        assert_eq!(vec![0, 1], Solution::even_odd_bit(2));
    }
}
