/**
 * [3184] Count Pairs That Form a Complete Day I
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn count_complete_day_pairs(hours: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        let mut result = 0;

        for hour in hours {
            let hour = hour % 24;
            if let Some(&c) = map.get(&((24 - hour) % 24)) {
                result += c;
            }
            let entry = map.entry(hour).or_insert(0);
            *entry += 1;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3184() {
        assert_eq!(3, Solution::count_complete_day_pairs(vec![72, 48, 24, 3]));
        assert_eq!(2, Solution::count_complete_day_pairs(vec![12, 12, 30, 24, 24]));
    }
}
