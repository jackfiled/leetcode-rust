/**
 * [81] Search in Rotated Sorted Array II
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        if nums.len() == 1 {
            return target == nums[0];
        }

        let (mut left, mut right) = (0, nums.len());

        // 搜索范围[left, right)
        while left < right {
            let middle = (right - left) / 2 + left;

            if nums[middle] == target {
                return true;
            }

            if nums[middle] > nums[0] {
                // 左侧的数组是有序的
                if target >= nums[0] && target < nums[middle] {
                    right = Self::get_first_middle(&nums, middle);
                } else {
                    left = Self::get_last_middle(&nums, middle) + 1;
                }
            } else if nums[middle] == nums[0] {
                // 有可能nums[middle] == nums[0]
                let first_middle = Self::get_first_middle(&nums, middle);
                if first_middle == 0 {
                    // [0, middle] 都是同一个数
                    left = Self::get_last_middle(&nums, middle) + 1;
                } else {
                    // 反之那么[middle,]都是同一个数
                    right = first_middle;
                }
            } else {
                // 右侧数组是有序的
                if target > nums[middle] && target <= nums[nums.len() - 1] {
                    left = Self::get_last_middle(&nums, middle) + 1;
                } else {
                    right = Self::get_first_middle(&nums, middle);
                }
            }
        }

        false
    }

    fn get_first_middle(nums: &Vec<i32>, mut middle: usize) -> usize {
        while middle > 0 && nums[middle - 1] == nums[middle] {
            middle -= 1;
        }

        middle
    }

    fn get_last_middle(nums: &Vec<i32>, mut middle: usize) -> usize {
        while middle < nums.len() - 1 && nums[middle + 1] == nums[middle] {
            middle += 1;
        }

        middle
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_81() {
        assert!(Solution::search(
            vec![1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            2
        ));
        assert!(Solution::search(
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1],
            2
        ));
        assert!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 0));
        assert!(!Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3));
    }
}
