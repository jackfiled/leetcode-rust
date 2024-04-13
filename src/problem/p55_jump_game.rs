/**
 * [55] Jump Game
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max_reach = 0;

        for i in 0..nums.len() {
            if i > max_reach {
                return false;
            }

            max_reach = max_reach.max(i + nums[i] as usize);

            if max_reach >= nums.len() - 1 {
                return true;
            }
        } 

        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_55() {
        assert!(Solution::can_jump(vec![2, 3, 1, 1, 4]));
        assert!(!Solution::can_jump(vec![3, 2, 1, 0, 4]));
    }
}
