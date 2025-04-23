/**
 * [1399] Count Largest Group
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut map = HashMap::new();
        (1..=n)
            .into_iter()
            .map(|mut x| {
                let mut result = 0;

                while x > 0 {
                    result += x % 10;
                    x = x / 10;
                }

                result
            })
            .fold(&mut map, |map, i| {
                let entry = map.entry(i).or_insert(0);
                *entry += 1;

                map
            });

        let max_count = map.values().max().unwrap();
        map.values().filter(|x| x == &max_count).count() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1399() {
        assert_eq!(4, Solution::count_largest_group(13));
        assert_eq!(2, Solution::count_largest_group(2));
        assert_eq!(6, Solution::count_largest_group(15));
        assert_eq!(5, Solution::count_largest_group(24));
    }
}
