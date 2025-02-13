/**
 * [1742] Maximum Number of Balls in a Box
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
        let mut array = vec![0; 100];

        for mut i in low_limit..=high_limit {
            let mut sum = 0;
            while i > 0 {
                sum += i % 10;
                i = i / 10;
            }

            array[sum as usize] += 1;
        }

        *array.iter().max().unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1742() {
        assert_eq!(2, Solution::count_balls(1, 10));
        assert_eq!(2, Solution::count_balls(5, 15));
        assert_eq!(2, Solution::count_balls(19, 28));
    }
}
