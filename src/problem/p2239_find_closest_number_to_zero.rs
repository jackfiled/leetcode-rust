/**
 * [2239] Find Closest Number to Zero
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn find_closest_number(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(i32::MAX, |acc, v| {
            if v.abs() < acc.abs() {
                v
            } else if v.abs() == acc.abs() && v > acc {
                v
            } else {
                acc
            }
        })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2239() {
        assert_eq!(-1, Solution::find_closest_number(vec![-1, -1]));
        assert_eq!(1, Solution::find_closest_number(vec![-4, -2, 1, 4, 8]));
        assert_eq!(1, Solution::find_closest_number(vec![-2, -1, 1]));
    }
}
