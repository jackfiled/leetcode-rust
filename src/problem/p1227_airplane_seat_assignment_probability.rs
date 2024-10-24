/**
 * [1227] Airplane Seat Assignment Probability
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn nth_person_gets_nth_seat(n: i32) -> f64 {
        if n == 1 {
            1f64
        } else {
            0.5
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1227() {
        assert_eq!(1f64, Solution::nth_person_gets_nth_seat(1));
        assert_eq!(0.5, Solution::nth_person_gets_nth_seat(2));
    }
}
