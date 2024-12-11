/**
 * [2717] Semi-Ordered Permutation
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn semi_ordered_permutation(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let one_pos = nums
            .iter()
            .enumerate()
            .find_map(|(i, v)| if *v == 1 { Some(i) } else { None })
            .unwrap();
        let n_pos = nums
            .iter()
            .enumerate()
            .find_map(|(i, v)| if *v == n as i32 { Some(i) } else { None })
            .unwrap();

        let result = if one_pos < n_pos {
            (n - 1 - n_pos) + (one_pos - 0)
        } else {
            (n - 1 - n_pos) + (one_pos - 0) - 1
        } as i32;

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2717() {
        assert_eq!(2, Solution::semi_ordered_permutation(vec![2, 1, 4, 3]));
        assert_eq!(3, Solution::semi_ordered_permutation(vec![2, 4, 1, 3]));
        assert_eq!(0, Solution::semi_ordered_permutation(vec![1, 3, 4, 2, 5]));
    }
}
