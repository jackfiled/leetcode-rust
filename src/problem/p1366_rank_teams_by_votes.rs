/**
 * [1366] Rank Teams by Votes
 */
pub struct Solution {}

// submission codes start here
use std::cmp::Ordering;
use std::collections::HashMap;

impl Solution {
    pub fn rank_teams(votes: Vec<String>) -> String {
        let m = votes[0].len();
        let mut map = HashMap::with_capacity(m);

        for vote in votes {
            for (i, c) in vote.chars().enumerate() {
                let entry = map.entry(c).or_insert(vec![0; m]);
                entry[i] += 1;
            }
        }

        let mut result: Vec<char> = map.keys().map(|c| c.clone()).collect();

        result.sort_unstable_by(|a, b| {
            let (a_score, b_score) = (map.get(a).unwrap(), map.get(b).unwrap());

            for (i, j) in a_score.into_iter().zip(b_score.into_iter()) {
                match i.cmp(j) {
                    Ordering::Equal => {
                        continue;
                    }
                    Ordering::Greater => return Ordering::Less,
                    Ordering::Less => return Ordering::Greater,
                }
            }

            a.cmp(b)
        });

        result.iter().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1366() {
        assert_eq!(
            "ACB".to_owned(),
            Solution::rank_teams(vec_string!("ABC", "ACB", "ABC", "ACB", "ACB"))
        );
        assert_eq!(
            "XWYZ".to_owned(),
            Solution::rank_teams(vec_string!("WXYZ", "XYZW"))
        );
        assert_eq!(
            "ZMNAGUEDSJYLBOPHRQICWFXTVK".to_owned(),
            Solution::rank_teams(vec_string!("ZMNAGUEDSJYLBOPHRQICWFXTVK"))
        );
    }
}
