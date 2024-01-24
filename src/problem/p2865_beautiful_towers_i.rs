/**
 * [2865] Beautiful Towers I
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn maximum_sum_of_heights(max_heights: Vec<i32>) -> i64 {
        let mut result = 0;

        for peek in 0..max_heights.len() {
            let mut total_height = max_heights[peek] as i64;
            let mut heights = vec![0;max_heights.len()];
            heights[peek] = max_heights[peek];

            for i in (0..peek).rev() {
                heights[i] = max_heights[i].min(heights[i + 1]);
                total_height += heights[i] as i64;
            }

            for i in peek + 1..max_heights.len() {
                heights[i] = max_heights[i].min(heights[i - 1]);
                total_height += heights[i] as i64;
            }

            result = result.max(total_height);
        }


        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2865() {
        assert_eq!(Solution::maximum_sum_of_heights(vec![5,3,4,1,1]), 13);
        assert_eq!(Solution::maximum_sum_of_heights(vec![6,5,3,9,2,6]), 22);
        assert_eq!(Solution::maximum_sum_of_heights(vec![3,2,5,5,2,3]), 18);
    }
}
