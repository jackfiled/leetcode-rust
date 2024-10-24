/**
 * [26] Remove Duplicates from Sorted Array
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut now = 1;

        for i in 1..nums.len() {
            if nums[i] != nums[i - 1] {
                nums[now] = nums[i];
                now += 1;
            }
        }

        now as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_26() {}
}
