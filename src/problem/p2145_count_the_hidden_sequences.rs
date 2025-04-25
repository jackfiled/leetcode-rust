/**
 * [2145] Count the Hidden Sequences
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let mut min_value = 0i64;
        let mut max_value = 0i64;
        let mut last_value = 0i64;

        for i in differences.into_iter() {
            let value = last_value + i as i64;
            min_value = min_value.min(value);
            max_value = max_value.max(value);
            last_value = value;
        }

        let range = max_value - min_value;

        let result = (upper - lower + 1) as i64 - range;
        if result > 0 {
            result as i32
        } else {
            0
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2145() {
        assert_eq!(60, Solution::number_of_arrays(vec![-40], -46, 53));
        assert_eq!(2, Solution::number_of_arrays(vec![1, -3, 4], 1, 6));
        assert_eq!(4, Solution::number_of_arrays(vec![3, -4, 5, 1, -2], -4, 5));
        assert_eq!(0, Solution::number_of_arrays(vec![4, -7, 2], 3, 6));
    }
}
