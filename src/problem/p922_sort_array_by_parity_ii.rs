/**
 * [922] Sort Array By Parity II
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn sort_array_by_parity_ii(mut nums: Vec<i32>) -> Vec<i32> {
        let mut last_odd_pos = vec![];
        let mut last_even_pos = vec![];

        for i in 0..nums.len() {
            if i % 2 == 0 && nums[i] % 2 != 0 {
                if let Some(pos) = last_even_pos.pop() {
                    nums.swap(pos, i);
                } else {
                    last_odd_pos.push(i);
                }
            }
            if i % 2 != 0 && nums[i] % 2 == 0 {
                if let Some(pos) = last_odd_pos.pop() {
                    nums.swap(pos, i);
                } else {
                    last_even_pos.push(i);
                }
            }
        }

        nums
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_922() {
        assert_eq!(
            vec![0, 9, 2, 5, 6, 7, 8],
            // 0, 1, 2, 3, 4, 5, 6
            Solution::sort_array_by_parity_ii(vec![0, 8, 2, 6, 5, 7, 9])
        );
        assert_eq!(
            vec![4, 5, 2, 7],
            Solution::sort_array_by_parity_ii(vec![4, 2, 5, 7])
        );
        assert_eq!(vec![2, 3], Solution::sort_array_by_parity_ii(vec![2, 3]));
    }
}
