use std::thread::available_parallelism;

/**
 * [624] Maximum Distance in Arrays
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        // -10e4 <= array[i][j] <= 10e4
        let (mut min, mut max) = (10_000 + 1, -10_000 - 1);

        for array in arrays {
            result = result
                .max(*array.last().unwrap() - min)
                .max(max - *array.first().unwrap());

            min = min.min(*array.first().unwrap());
            max = max.max(*array.last().unwrap());
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_624() {
        assert_eq!(
            4,
            Solution::max_distance(vec![vec![1, 2, 3], vec![4, 5], vec![1, 2, 3]])
        );
        assert_eq!(0, Solution::max_distance(vec![vec![1], vec![1]]));
    }
}
