/**
 * [1552] Magnetic Force Between Two Balls
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn max_distance(mut position: Vec<i32>, m: i32) -> i32 {
        position.sort_unstable();

        let mut left = i32::MAX;
        for i in 0..position.len() - 1 {
            left = left.min(position[i + 1] - position[i]);
        }

        let mut right = position[position.len() - 1] - position[0];

        // [left, right]
        let mut result = 0;
        while left <= right {
            let middle = (right - left) / 2 + left;

            if Self::check(&position, m, middle) {
                result = middle;
                left = middle + 1;
            } else {
                right = middle - 1;
            }
        }

        result
    }

    fn check(position: &Vec<i32>, m: i32, dis: i32) -> bool {
        let mut pre = position[0];
        let mut count = 1;

        for i in 1..position.len() {
            if position[i] - pre >= dis {
                count += 1;
                pre = position[i];
            }
        }

        count >= m
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1552() {
        assert_eq!(3, Solution::max_distance(vec![1, 2, 3, 4, 7], 3));
        assert_eq!(
            999_999_999,
            Solution::max_distance(vec![5, 4, 3, 2, 1, 1_000_000_000], 2)
        );
    }
}
