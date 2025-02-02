/**
 * [598] Range Addition II
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn max_count(mut m: i32, mut n: i32, ops: Vec<Vec<i32>>) -> i32 {
        for array in ops {
            m = m.min(array[0]);
            n = n.min(array[1]);
        }

        m * n
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_598() {
        assert_eq!(4, Solution::max_count(3, 3, vec![vec![2, 2], vec![3, 3]]));
        assert_eq!(
            4,
            Solution::max_count(
                3,
                3,
                vec![
                    vec![2, 2],
                    vec![3, 3],
                    vec![3, 3],
                    vec![3, 3],
                    vec![2, 2],
                    vec![3, 3],
                    vec![3, 3],
                    vec![3, 3],
                    vec![2, 2],
                    vec![3, 3],
                    vec![3, 3],
                    vec![3, 3]
                ]
            )
        );
        assert_eq!(9, Solution::max_count(3, 3, vec![]));
    }
}
