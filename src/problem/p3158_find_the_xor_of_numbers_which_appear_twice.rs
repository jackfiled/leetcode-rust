/**
 * [3158] Find the XOR of Numbers Which Appear Twice
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        let mut set = HashSet::with_capacity(nums.len());

        let mut result = 0;

        for num in nums {
            set.insert(num);
            result ^= num;
        }

        for num in set {
            result ^= num;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3158() {
        assert_eq!(1, Solution::duplicate_numbers_xor(vec![1, 2, 1, 3]));
        assert_eq!(0, Solution::duplicate_numbers_xor(vec![1, 2, 3]));
        assert_eq!(3, Solution::duplicate_numbers_xor(vec![1, 2, 2, 1]));
    }
}
