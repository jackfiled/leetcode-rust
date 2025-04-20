/**
 * [781] Rabbits in Forest
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut map = HashMap::new();

        for i in answers.into_iter() {
            let entry = map.entry(i).or_insert(0);
            *entry += 1;
        }

        let mut result = 0;

        for (k, mut v) in map.into_iter() {
            // 同种颜色兔子最多有k + 1个
            let max_count = k + 1;

            while v > 0 {
                result += max_count;
                v -= max_count;
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
    fn test_781() {
        assert_eq!(5, Solution::num_rabbits(vec![1, 0, 1, 0, 0]));
        assert_eq!(5, Solution::num_rabbits(vec![1, 1, 2]));
        assert_eq!(11, Solution::num_rabbits(vec![10, 10, 10]))
    }
}
