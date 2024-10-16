/**
 * [3194] Minimum Average of Smallest and Largest Elements
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn minimum_average(nums: Vec<i32>) -> f64 {
        let mut nums: Vec<f64> = nums.into_iter().map(|x| x as f64).collect();
        nums.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let n = nums.len();

        let mut result = f64::MAX;

        for i in 0..n / 2 {
            result = result.min((nums[i] + nums[n - i - 1]) / 2f64);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3194() {
        assert_eq!(5.5, Solution::minimum_average(vec![7, 8, 3, 4, 15, 13, 4, 1]));
        assert_eq!(5.5, Solution::minimum_average(vec![1, 9, 8, 3, 10, 5]));
        assert_eq!(5.0, Solution::minimum_average(vec![1, 2, 3, 7, 8, 9]));
    }
}
