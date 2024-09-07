/**
 * [3177] Find the Maximum Length of a Good Subsequence II
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
    fn test_3177() {
    }
}
