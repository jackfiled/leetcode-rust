/**
 * [2270] Number of Ways to Split Array
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let nums: Vec<i64> = nums.into_iter().map(|x| x as i64).collect();
        let sum = nums.iter().sum::<i64>();

        let mut current = 0;
        let mut result = 0;

        for i in 0..nums.len() - 1 {
            current += nums[i];

            if current * 2 >= sum {
                result += 1;
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
    fn test_2270() {
        assert_eq!(2, Solution::ways_to_split_array(vec![10, 4, -8, 7]));
        assert_eq!(2, Solution::ways_to_split_array(vec![2, 3, 1, 0]));
    }
}
