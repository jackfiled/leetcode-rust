/**
 * [2545] Sort the Students by Their Kth Score
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn sort_the_students(score: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;

        let mut target_scores: Vec<(i32, usize)> =
            score.iter().enumerate().map(|(i, v)| (v[k], i)).collect();

        target_scores.sort_unstable_by(|a, b| b.0.cmp(&a.0));

        target_scores
            .into_iter()
            .map(|x| score[x.1].clone())
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2545() {
        assert_eq!(
            vec![vec![7, 5, 11, 2], vec![10, 6, 9, 1], vec![4, 8, 3, 15]],
            Solution::sort_the_students(
                vec![vec![10, 6, 9, 1], vec![7, 5, 11, 2], vec![4, 8, 3, 15]],
                2
            )
        );
    }
}
