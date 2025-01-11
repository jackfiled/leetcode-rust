/**
 * [3270] Find the Key of the Numbers
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn generate_key(num1: i32, num2: i32, num3: i32) -> i32 {
        let mut array = [num1, num2, num3];
        let mut result = 0;

        for i in 0..4 {
            result += array.iter().map(|x| *x % 10).min().unwrap() * 10_i32.pow(i);

            for i in 0..3 {
                array[i] /= 10;
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
    fn test_3270() {
        assert_eq!(0, Solution::generate_key(1, 10, 1000));
        assert_eq!(777, Solution::generate_key(987, 879, 798));
        assert_eq!(1, Solution::generate_key(1, 2, 3));
    }
}
