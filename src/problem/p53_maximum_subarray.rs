/**
 * [53] Maximum Subarray
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut pre = 0;
        let mut result = nums[0];

        for i in nums {
            pre = i.max(pre + i);
            result = result.max(pre);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_53() {
    }
}
