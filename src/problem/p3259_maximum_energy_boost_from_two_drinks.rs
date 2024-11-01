/**
 * [3259] Maximum Energy Boost From Two Drinks
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn max_energy_boost(energy_drink_a: Vec<i32>, energy_drink_b: Vec<i32>) -> i64 {
        let n = energy_drink_a.len();
        let mut dp_drink_a = vec![0; n];
        let mut dp_drink_b = vec![0; n];

        for i in 0..n {
            if i >= 2 {
                dp_drink_a[i] = dp_drink_a[i - 1].max(dp_drink_b[i - 2]) + energy_drink_a[i] as i64;
                dp_drink_b[i] = dp_drink_b[i - 1].max(dp_drink_a[i - 2]) + energy_drink_b[i] as i64;
            } else if i >= 1 {
                dp_drink_a[i] = dp_drink_a[i - 1] + energy_drink_a[i] as i64;
                dp_drink_b[i] = dp_drink_b[i - 1] + energy_drink_b[i] as i64;
            } else {
                dp_drink_a[i] = energy_drink_a[i] as i64;
                dp_drink_b[i] = energy_drink_b[i] as i64;
            }
        }

        dp_drink_a[n - 1].max(dp_drink_b[n - 1])
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3259() {
        assert_eq!(5, Solution::max_energy_boost(vec![1, 3, 1], vec![3, 1, 1]));
        assert_eq!(7, Solution::max_energy_boost(vec![4, 1, 1], vec![1, 1, 3]));
    }
}
