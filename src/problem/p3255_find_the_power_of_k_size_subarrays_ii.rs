/**
 * [3255] Find the Power of K-Size Subarrays II
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let n = nums.len();

        let mut count = 0;
        for i in 0..(k - 1) {
            if nums[i] + 1 == nums[i + 1] {
                count += 1;
            }
        }

        let mut result = Vec::with_capacity(n - k + 1);
        result.push(if count == k - 1 { nums[k - 1] } else { -1 });

        for i in k..n {
            if nums[i - 1] + 1 == nums[i] {
                count += 1;
            }

            if nums[i - k] + 1 == nums[i - k + 1] {
                count -= 1;
            }

            result.push(if count == k - 1 { nums[i] } else { -1 });
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3255() {
        assert_eq!(
            vec![3, 4, -1, -1, -1],
            Solution::results_array(vec![1, 2, 3, 4, 3, 2, 5], 3)
        );
        assert_eq!(
            vec![-1, -1],
            Solution::results_array(vec![2, 2, 2, 2, 2], 4)
        );
        assert_eq!(
            vec![-1, 3, -1, 3, -1],
            Solution::results_array(vec![3, 2, 3, 2, 3, 2], 2)
        );
    }
}
