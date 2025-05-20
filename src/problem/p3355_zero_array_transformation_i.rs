/**
 * [3355] Zero Array Transformation I
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
        let n = nums.len();

        let mut difference_array = vec![0; n + 1];

        for query in queries {
            let left = query[0] as usize;
            let right = query[1] as usize;

            difference_array[left] += 1;
            difference_array[right + 1] -= 1;
        }

        let mut prefix_sum = difference_array[0];

        for i in 1..=n {
            if prefix_sum < nums[i - 1] {
                return false;
            }

            prefix_sum += difference_array[i];
        }

        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3355() {
        assert!(Solution::is_zero_array(vec![1, 0, 1], vec![vec![0, 2]]));
        assert!(!Solution::is_zero_array(
            vec![4, 3, 2, 1],
            vec![vec![1, 3], vec![0, 2]]
        ));
    }
}
