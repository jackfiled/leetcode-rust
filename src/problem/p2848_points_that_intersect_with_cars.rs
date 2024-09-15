/**
 * [2848] Points That Intersect With Cars
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn number_of_points(nums: Vec<Vec<i32>>) -> i32 {
        let mut nums: Vec<(i32, i32)> = nums.iter().map(|i| (i[0], i[1])).collect();
        nums.sort();

        let mut result = 0;
        let mut last_end = 0;

        for (start, end) in nums {
            if last_end < start {
                result += (end - start) + 1;
                last_end = end;
            } else if end > last_end {
                result += (end - last_end);
                last_end = end;
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2848() {
        assert_eq!(7, Solution::number_of_points(vec![vec![3, 6], vec![1, 5], vec![4, 7]]));
        assert_eq!(7, Solution::number_of_points(vec![vec![1, 3], vec![5, 8]]));
        assert_eq!(8, Solution::number_of_points(vec![vec![4, 4], vec![9, 10], vec![9, 10], vec![3, 8]]));
    }
}
