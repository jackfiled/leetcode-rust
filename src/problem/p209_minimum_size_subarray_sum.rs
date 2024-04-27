/**
 * [209] Minimum Size Subarray Sum
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let length = nums.len();

        let (mut start, mut end) = (0, 0);
        let mut result = i32::MAX;

        let mut sum = 0;
        while end < length {
            sum += nums[end];

            while sum >= target {
                result = result.min((end - start + 1) as i32);
                sum -= nums[start];
                start += 1;
            }

            end += 1;
        }

        if result == i32::MAX {
            0
        } else {
            result
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_209() {
    }
}
