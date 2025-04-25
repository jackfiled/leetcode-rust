/**
 * [2012] Sum of Beauty in the Array
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn sum_of_beauties(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut prefix = vec![0; n];
        let mut suffix = vec![i32::MAX; n];

        for i in 0..n {
            if i > 0 {
                prefix[i] = prefix[i - 1].max(nums[i - 1]);
                suffix[n - i - 1] = suffix[n - i].min(nums[n - i]);
            }
        }

        (1..n - 1)
            .into_iter()
            .map(|x| {
                if nums[x] > prefix[x] && nums[x] < suffix[x] {
                    2
                } else if nums[x] > nums[x - 1] && nums[x] < nums[x + 1] {
                    1
                } else {
                    0
                }
            })
            .sum()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2012() {
        assert_eq!(1, Solution::sum_of_beauties(vec![2, 4, 6, 4]));
        assert_eq!(2, Solution::sum_of_beauties(vec![1, 2, 3]));
        assert_eq!(0, Solution::sum_of_beauties(vec![3, 2, 1]));
    }
}
