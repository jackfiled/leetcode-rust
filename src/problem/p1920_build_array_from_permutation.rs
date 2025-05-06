/**
 * [1920] Build Array from Permutation
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        nums.iter().map(|x| nums[*x as usize]).collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1920() {
        assert_eq!(
            vec![0, 1, 2, 4, 5, 3],
            Solution::build_array(vec![0, 2, 1, 5, 3, 4])
        );
        assert_eq!(
            vec![4, 5, 0, 1, 2, 3],
            Solution::build_array(vec![5, 0, 1, 2, 3, 4])
        );
    }
}
