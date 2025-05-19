/**
 * [3024] Type of Triangle
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn triangle_type(nums: Vec<i32>) -> String {
        if nums[0] + nums[1] <= nums[2]
            || nums[0] + nums[2] <= nums[1]
            || nums[1] + nums[2] <= nums[0]
        {
            return "none".to_string();
        }

        if nums[0] == nums[1] || nums[0] == nums[2] || nums[1] == nums[2] {
            if nums[0] == nums[1] && nums[1] == nums[2] {
                "equilateral".to_string()
            } else {
                "isosceles".to_string()
            }
        } else {
            "scalene".to_string()
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3024() {
        assert_eq!("equilateral", Solution::triangle_type(vec![3, 3, 3]));
        assert_eq!("scalene", Solution::triangle_type(vec![3, 4, 5]));
    }
}
