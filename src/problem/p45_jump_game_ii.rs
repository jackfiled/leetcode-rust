/**
 * [45] Jump Game II
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut max_position = 0;
        let mut current_position = 0;

        // 注意这里不能遍历到数组最后
        for i in 0..nums.len() - 1 {
            max_position = max_position.max(i as i32 + nums[i]);

            if i as i32 == current_position {
                current_position = max_position;
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
    fn test_45() {
        assert_eq!(2, Solution::jump(vec![2, 3, 1, 1, 4]));
        assert_eq!(2, Solution::jump(vec![2, 3, 0, 1, 4]));
    }
}
