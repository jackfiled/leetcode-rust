/**
 * [3356] Zero Array Transformation II
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let (mut left, mut right) = (0, queries.len());

        if !Self::check(&nums, &queries, right) {
            return -1;
        }

        while left < right {
            let k = (left + right) / 2;
            if Self::check(&nums, &queries, k) {
                right = k;
            } else {
                left = k + 1;
            }
        }

        left as i32
    }

    fn check(nums: &Vec<i32>, queries: &Vec<Vec<i32>>, k: usize) -> bool {
        let n = nums.len();
        let mut difference_array = vec![0; n + 1];

        for query in queries[..k].iter() {
            difference_array[query[0] as usize] += query[2];
            difference_array[query[1] as usize + 1] -= query[2];
        }

        let mut prefix_sum = difference_array[0];

        for (i, &v) in difference_array[1..].iter().enumerate() {
            if prefix_sum < nums[i] {
                return false;
            }

            prefix_sum += v;
        }

        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3356() {
        assert_eq!(
            2,
            Solution::min_zero_array(
                vec![2, 0, 2],
                vec![vec![0, 2, 1], vec![0, 2, 1], vec![1, 1, 3]]
            )
        );
        assert_eq!(
            -1,
            Solution::min_zero_array(vec![4, 3, 2, 1], vec![vec![1, 3, 2], vec![0, 2, 1]])
        );
    }
}
