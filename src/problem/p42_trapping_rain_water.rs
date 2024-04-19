/**
 * [42] Trapping Rain Water
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let length = height.len();
        let mut left_max = vec![height[0]; length];
        let mut right_max = vec![height[length - 1]; length];

        for i in (1..length) {
            left_max[i] = left_max[i - 1].max(height[i]);
        }

        for i in (0..length - 1).rev() {
            right_max[i] = right_max[i + 1].max(height[i]);
        }

        let mut result = 0;

        for i in 0..length {
            result += left_max[i].min(right_max[i]) - height[i];
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_42() {
        assert_eq!(6, Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]));
    }
}
