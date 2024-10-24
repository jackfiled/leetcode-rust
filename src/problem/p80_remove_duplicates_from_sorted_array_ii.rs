/**
 * [80] Remove Duplicates from Sorted Array II
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut now = 1;
        let mut count = 1;

        for i in 1..nums.len() {
            if nums[i - 1] != nums[i] {
                count = 1;
                nums[now] = nums[i];
                now += 1;
            } else {
                if count < 2 {
                    nums[now] = nums[i];
                    now += 1;
                }

                count += 1;
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
    fn test_80() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        assert_eq!(5, Solution::remove_duplicates(&mut nums));
        assert_eq!(vec![1, 1, 2, 2, 3, 3], nums);
    }
}
