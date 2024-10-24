/**
 * [1686] Stone Game VI
 */
pub struct Solution {}

// submission codes start here

struct Stone {
    sum: i32,
    alice: i32,
    bob: i32,
}

impl Solution {
    pub fn stone_game_vi(alice_values: Vec<i32>, bob_values: Vec<i32>) -> i32 {
        let mut stones: Vec<Stone> = alice_values
            .iter()
            .zip(bob_values.iter())
            .map(|(a, b)| Stone {
                sum: *a + *b,
                alice: *a,
                bob: *b,
            })
            .collect();

        stones.sort_unstable_by(|a, b| b.sum.cmp(&a.sum));

        let alice_sum: i32 = stones.iter().step_by(2).map(|s| s.alice).sum();
        let bob_sum: i32 = stones.iter().skip(1).step_by(2).map(|s| s.bob).sum();

        if alice_sum > bob_sum {
            1
        } else if alice_sum < bob_sum {
            -1
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
    fn test_1686() {
        assert_eq!(Solution::stone_game_vi(vec![1, 3], vec![2, 1]), 1);
    }
}
