/**
 * [1007] Minimum Domino Rotations For Equal Row
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        let sequence: Vec<(i32, i32)> = tops.into_iter().zip(bottoms.into_iter()).collect();

        // 应该首先确定相同的值是多少
        let mut value = 0;
        if sequence[1..]
            .iter()
            .all(|(t, b)| *t == sequence[0].0 || *b == sequence[0].0)
        {
            value = sequence[0].0;
        }
        if sequence[1..]
            .iter()
            .all(|(t, b)| *t == sequence[0].1 || *b == sequence[0].1)
        {
            value = sequence[0].1;
        }

        if value == 0 {
            return -1;
        }

        sequence
            .iter()
            .filter(|(t, b)| *t != value)
            .count()
            .min(sequence.iter().filter(|(t, b)| *b != value).count()) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1007() {
        assert_eq!(
            1,
            Solution::min_domino_rotations(
                vec![1, 2, 1, 1, 1, 2, 2, 2],
                vec![2, 1, 2, 2, 2, 2, 2, 2]
            )
        );
        assert_eq!(
            2,
            Solution::min_domino_rotations(vec![2, 1, 2, 4, 2, 2], vec![5, 2, 6, 2, 3, 2])
        );
        assert_eq!(
            -1,
            Solution::min_domino_rotations(vec![3, 5, 1, 2, 3], vec![3, 6, 3, 3, 4])
        );
    }
}
