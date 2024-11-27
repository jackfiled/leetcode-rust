/**
 * [3208] Alternating Groups II
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
        let n = colors.len();
        let k = k as usize - 1;

        let mut result = 0;
        let mut count = 0;

        for i in n..(2 * n + k - 1) {
            if colors[i % n] != colors[(i + 1) % n] {
                count += 1;
            } else {
                count = 0;
            }

            if count >= k {
                result += 1;
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3208() {
        assert_eq!(1, Solution::number_of_alternating_groups(vec![0, 0, 1], 3));
        assert_eq!(1, Solution::number_of_alternating_groups(vec![0, 1, 1], 3));
        assert_eq!(
            3,
            Solution::number_of_alternating_groups(vec![0, 1, 0, 1, 0], 3)
        );
        assert_eq!(
            2,
            Solution::number_of_alternating_groups(vec![0, 1, 0, 0, 1, 0, 1], 6)
        );
    }
}
