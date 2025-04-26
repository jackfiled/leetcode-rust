/**
 * [2444] Count Subarrays With Fixed Bounds
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let mut result = 0;
        let (mut border, mut last_min, mut last_max) = (None, None, None);

        for i in 0..nums.len() {
            if nums[i] < min_k || nums[i] > max_k {
                last_max = None;
                last_min = None;
                border = Some(i);
            }

            if nums[i] == min_k {
                last_min = Some(i);
            }

            if nums[i] == max_k {
                last_max = Some(i);
            }

            if let Some(last_min) = last_min {
                if let Some(last_max) = last_max {
                    let current = if let Some(border) = border {
                        last_min.min(last_max) - border
                    } else {
                        last_min.min(last_max) + 1
                    };

                    result += current as i64
                }
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
    fn test_2444() {
        assert_eq!(2, Solution::count_subarrays(vec![1, 3, 5, 2, 7, 5], 1, 5));
        assert_eq!(10, Solution::count_subarrays(vec![1, 1, 1, 1], 1, 1));
    }
}
