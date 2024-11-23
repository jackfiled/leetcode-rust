/**
 * [3238] Find the Number of Winning Players
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn winning_player_count(n: i32, pick: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        let n = n as usize;

        let mut player = vec![HashMap::new(); n];
        for p in pick {
            let (p, b) = (p[0] as usize, p[1]);

            let mut entry = player[p].entry(b).or_insert(0);
            *entry += 1;
        }

        player
            .iter()
            .enumerate()
            .filter_map(|(i, map)| {
                if map.iter().any(|(_, v)| *v > i) {
                    Some(())
                } else {
                    None
                }
            })
            .count() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3238() {
        assert_eq!(
            2,
            Solution::winning_player_count(
                4,
                vec![
                    vec![0, 0],
                    vec![1, 0],
                    vec![1, 0],
                    vec![2, 1],
                    vec![2, 1],
                    vec![2, 0]
                ]
            )
        );
    }
}
