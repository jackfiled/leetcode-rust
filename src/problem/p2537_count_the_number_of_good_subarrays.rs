/**
 * [2537] Count the Number of Good Subarrays
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
        let mut map = HashMap::new();
        let n = nums.len();

        let mut result = 0i64;
        let mut count = 0;
        let (mut left, mut right) = (0, 0);

        while left < n {
            while right < n && count < k {
                let entry = map.entry(nums[right]).or_insert(0);
                count += *entry;
                *entry += 1;

                right += 1;
            }

            if count < k && right >= n {
                // 拼尽全力无法战胜
                break;
            }

            // 此时已经遇到了一个合格的序列
            // 注意序列可以多余的包含右侧的所有元素
            result += (n - right) as i64 + 1;

            // 右移left
            let entry = map.entry(nums[left]).or_insert(0);
            count -= *entry - 1;
            *entry -= 1;
            left += 1;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2537() {
        assert_eq!(4, Solution::count_good(vec![3, 1, 4, 3, 2, 2, 4], 2));
        assert_eq!(1, Solution::count_good(vec![1, 1, 1, 1, 1], 10));
    }
}
