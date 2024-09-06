/**
 * [3176] Find the Maximum Length of a Good Subsequence I
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;

        let k = k as usize;
        let mut map = HashMap::with_capacity(nums.len());
        let mut mx = vec![0; k + 2];

        for x in nums {
            let entry = map.entry(x).or_insert(vec![0; k + 1]);

            for j in (0..=k).rev() {
                entry[j] = entry[j].max(mx[j]) + 1;
                mx[j + 1] = mx[j + 1].max(entry[j]);
            }
        }

        mx[k + 1]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3176() {
        assert_eq!(4, Solution::maximum_length(vec![1, 2, 1, 1, 3], 2));
        assert_eq!(2, Solution::maximum_length(vec![1,2,3,4,5,1], 0));
    }
}
