/**
 * [34] Find First and Last Position of Element in Sorted Array
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 0 {
            return vec![-1, -1];
        }

        let (mut left, mut right) = (0, nums.len());

        while left < right {
            let middle = (right - left) / 2 + left;

            if nums[middle] < target {
                left = middle + 1;
            } else {
                right = middle;
            }
        }

        if left >= nums.len() || nums[left] != target {
            return vec![-1, -1];
        }

        let mut l = left;

        while l >= 1 && nums[l - 1] == target {
            l -= 1;
        }

        let mut r = left;

        while r + 1 < nums.len() && nums[r + 1] == target {
            r += 1;
        }

        vec![l as i32, r as i32]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_34() {
        assert_eq!(
            vec![3, 4],
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8)
        );
        assert_eq!(vec![-1, -1], Solution::search_range(vec![2, 2], 3));
        assert_eq!(vec![0, 0], Solution::search_range(vec![1], 1));
        assert_eq!(vec![0, 1], Solution::search_range(vec![2, 2], 2));
    }
}
