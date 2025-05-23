/**
 * [3362] Zero Array Transformation III
 */
pub struct Solution {}

// submission codes start here
use std::collections::BinaryHeap;

impl Solution {
    pub fn max_removal(nums: Vec<i32>, mut queries: Vec<Vec<i32>>) -> i32 {
        queries.sort_unstable_by(|a, b| a[0].cmp(&b[0]));

        let mut heap = BinaryHeap::new();
        let mut difference_array = vec![0; nums.len() + 1];
        let mut prefix_sum = 0;
        let mut pos = 0;

        for i in 0..nums.len() {
            prefix_sum += difference_array[i];

            while pos < queries.len() && queries[pos][0] as usize == i {
                heap.push(queries[pos][1] as usize);
                pos += 1;
            }

            while prefix_sum < nums[i] {
                if let Some(&right) = heap.peek() {
                    if right >= i {
                        prefix_sum += 1;
                        difference_array[right + 1] -= 1;
                        heap.pop();
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            if prefix_sum < nums[i] {
                return -1;
            }
        }

        heap.len() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3362() {
        assert_eq!(
            1,
            Solution::max_removal(vec![2, 0, 2], vec![vec![0, 2], vec![0, 2], vec![1, 1]])
        );
        assert_eq!(
            2,
            Solution::max_removal(
                vec![1, 1, 1, 1],
                vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![1, 2]]
            )
        );
        assert_eq!(
            -1,
            Solution::max_removal(vec![1, 2, 3, 4], vec![vec![0, 3]])
        );
    }
}
