/**
 * [3134] Find the Median of the Uniqueness Array
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn median_of_uniqueness_array(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i64;
        let median = (n * (n + 1) / 2 + 1) / 2;

        // 检查数组中不同元素数目小于等于t的连续子数组是否大于等于median
        let check = |t: i64| -> bool {
            let mut map = HashMap::new();
            let mut total = 0;

            let mut j = 0;
            for i in 0..n {
                let entry = map.entry(nums[i as usize]).or_insert(0);
                *entry += 1;

                while map.len() as i64 > t {
                    let entry = map.get_mut(&nums[j]).unwrap();
                    *entry -= 1;

                    if *entry == 0 {
                        map.remove(&nums[j]);
                    }

                    j += 1;
                }

                total += i - j as i64 + 1;
            }

            total >= median
        };

        let mut result = 0;
        let (mut left, mut right) = (1, n);

        while left <= right {
            let middle = (left + right) / 2;

            if check(middle) {
                result = middle;
                right = middle - 1;
            } else {
                left = middle + 1;
            }
        }

        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3134() {
        assert_eq!(1, Solution::median_of_uniqueness_array(vec![1, 2, 3]));
        assert_eq!(2, Solution::median_of_uniqueness_array(vec![3, 4, 3, 4, 5]));
    }
}
