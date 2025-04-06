/**
 * [368] Largest Divisible Subset
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();
        let n = nums.len();

        let mut dp = vec![1; n];
        let mut max_size = 1;
        let mut max_value = nums[0];

        for i in 1..n {
            for j in 0..i {
                if nums[i] % nums[j] == 0 {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }

            if dp[i] > max_size {
                max_size = dp[i];
                max_value = nums[i];
            }
        }

        // 获得求得的最大子集
        let mut result = vec![];

        if max_size == 1 {
            result.push(nums[0]);
            return result;
        }

        for i in (0..n).rev() {
            if dp[i] == max_size && max_value % nums[i] == 0 {
                result.push(nums[i]);
                max_value = nums[i];
                max_size -= 1;
            }

            if max_size <= 0 {
                break;
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_368() {
        assert_array_unorder_equal!(vec![1], Solution::largest_divisible_subset(vec![1]));
        assert_array_unorder_equal!(
            vec![1, 2],
            Solution::largest_divisible_subset(vec![1, 2, 3])
        );
        assert_array_unorder_equal!(
            vec![1, 2, 4, 8],
            Solution::largest_divisible_subset(vec![1, 2, 4, 8])
        );
    }
}
