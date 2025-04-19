/**
 * [2563] Count the Number of Fair Pairs
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        nums.sort_unstable();
        let mut result = 0;

        for i in 0..nums.len() {
            let l = nums[0..i].partition_point(|&x| x + nums[i] < lower);
            let r = nums[0..i].partition_point(|&x| x + nums[i] <= upper);

            result += (r - l);
        }

        result as i64
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2563() {
        assert_eq!(6, Solution::count_fair_pairs(vec![0, 1, 7, 4, 4, 5], 3, 6));
        assert_eq!(1, Solution::count_fair_pairs(vec![1, 7, 9, 2, 5], 11, 11));
    }
}
