/**
 * [365] Water and Jug Problem
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn can_measure_water(jug1_capacity: i32, jug2_capacity: i32, target_capacity: i32) -> bool {
        if jug1_capacity + jug2_capacity < target_capacity {
            return false;
        }

        if jug1_capacity == 0 || jug2_capacity == 0 {
            return target_capacity == 0 || jug1_capacity + jug2_capacity == target_capacity;
        }

        target_capacity % Solution::gcd(jug1_capacity, jug2_capacity) == 0
    }

    fn gcd(a: i32, b: i32) -> i32 {
        let (mut a, mut b) = (a, b);

        while b > 0 {
            let t = a % b;
            a = b;
            b = t;
        }

        a
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_365() {
        assert!(Solution::can_measure_water(3, 5, 4));
        assert!(!Solution::can_measure_water(2, 6, 5));
        assert!(Solution::can_measure_water(1, 2, 3));
    }
}
