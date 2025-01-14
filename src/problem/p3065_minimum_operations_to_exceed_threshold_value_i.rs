/**
 * [3065] Minimum Operations to Exceed Threshold Value I
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn min_operations(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();

        let mut result = 0;

        for i in nums {
            if i < k {
                result += 1;
            } else {
                break;
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
    fn test_3065() {
        assert_eq!(3, Solution::min_operations(vec![2, 11, 10, 1, 3], 10));
        assert_eq!(0, Solution::min_operations(vec![1, 1, 2, 4, 9], 1));
        assert_eq!(4, Solution::min_operations(vec![1, 1, 2, 4, 9], 9));
    }
}
