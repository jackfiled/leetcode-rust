/**
 * [3117] Minimum Sum of Values by Dividing Array
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;

const INFINITE: i32 = (1 << 20) - 1;

impl Solution {
    pub fn minimum_value_sum(nums: Vec<i32>, and_values: Vec<i32>) -> i32 {
        let (n, m) = (nums.len(), and_values.len());
        let mut memory = vec![HashMap::new(); m * n];

        let result = Self::dfs(&mut memory, 0, 0, INFINITE, &nums, &and_values);
        if result == INFINITE {
            -1
        } else {
            result
        }
    }

    fn dfs(
        memory: &mut Vec<HashMap<i32, i32>>,
        i: usize,
        j: usize,
        current: i32,
        nums: &Vec<i32>,
        and_values: &Vec<i32>,
    ) -> i32 {
        let (n, m) = (nums.len(), and_values.len());
        let key = m * i + j;

        if i == n && j == m {
            return 0;
        }
        if i == n || j == m {
            return INFINITE;
        }
        if let Some(r) = memory[key].get(&current) {
            return *r;
        }

        let current = current & nums[i];
        if current & and_values[j] < and_values[j] {
            return INFINITE;
        }

        let mut result = Self::dfs(memory, i + 1, j, current, nums, and_values);
        if current == and_values[j] {
            result =
                result.min(Self::dfs(memory, i + 1, j + 1, INFINITE, nums, and_values) + nums[i]);
        }

        memory[key].insert(current, result);
        return result;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3117() {
        assert_eq!(
            12,
            Solution::minimum_value_sum(vec![1, 4, 3, 3, 2], vec![0, 3, 3, 2])
        );
    }
}
