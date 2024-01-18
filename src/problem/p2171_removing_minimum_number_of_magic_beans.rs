/**
 * [2171] Removing Minimum Number of Magic Beans
 */
pub struct Solution {}


// submission codes start here

use std::cmp::min;
impl Solution {
    pub fn minimum_removal(beans: Vec<i32>) -> i64 {
        let mut beans = beans;
        beans.sort_unstable();

        let mut sum = 0i64;
        for i in &beans {
            sum += *i as i64;
        }

        let mut result = i64::MAX;
        for (index, value) in (&beans).iter().enumerate() {
            result = min(result,
                         sum - (*value as i64) * (beans.len() - index) as i64);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2171() {
        assert_eq!(Solution::minimum_removal(vec![4,1,6,5]), 4);
        assert_eq!(Solution::minimum_removal(vec![2,10,3,2]), 7);
    }
}
