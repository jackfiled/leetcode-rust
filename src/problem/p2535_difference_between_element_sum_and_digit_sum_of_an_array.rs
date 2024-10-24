/**
 * [2535] Difference Between Element Sum and Digit Sum of an Array
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
        let mut result = 0;

        for num in nums {
            let mut num = num;
            let mut base = 1;

            while num > 0 {
                let digit = num % 10;

                result += digit * base - digit;

                base *= 10;
                num = num / 10;
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
    fn test_2535() {
        assert_eq!(9, Solution::difference_of_sum(vec![1, 15, 6, 3]));
        assert_eq!(0, Solution::difference_of_sum(vec![1, 2, 3, 4]));
    }
}
