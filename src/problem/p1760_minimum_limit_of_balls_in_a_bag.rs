/**
 * [1760] Minimum Limit of Balls in a Bag
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
        let nums = nums.into_iter().map(|x| x as i64).collect::<Vec<i64>>();
        let max_operations = max_operations as i64;
        let max_num = *nums.iter().max().unwrap();

        let mut left = 1; // check(left) == false
        let mut right = max_num; // check(right) == true

        // [left, right)
        while left < right {
            let middle = (right - left) / 2 + left;
            if nums.iter().map(|x| (*x - 1) / middle).sum::<i64>() <= max_operations {
                right = middle; // [left, middle)
            } else {
                left = middle + 1; // [middle + 1, right)
            }
        }

        left as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1760() {
        assert_eq!(2, Solution::minimum_size(vec![2, 4, 8, 2], 4));
        assert_eq!(3, Solution::minimum_size(vec![9], 2));
        assert_eq!(7, Solution::minimum_size(vec![7, 17], 2));
    }
}
