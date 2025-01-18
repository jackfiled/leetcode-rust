/**
 * [3287] Find the Maximum Sequence Value of Array
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashSet;

impl Solution {
    pub fn max_value(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let length = nums.len();
        let search = |array: &Vec<i32>| {
            let mut dp = Vec::with_capacity(array.len());
            let mut prev = vec![HashSet::new(); k + 1];
            prev[0].insert(0);

            for i in 0..array.len() {
                for j in (0..=(i + 1).min(k - 1)).rev() {
                    // 使用split_at_mut以通过借用检查
                    let (first, second) = prev.split_at_mut(j + 1);
                    for x in first[j].iter().map(|x| *x | array[i]) {
                        second[0].insert(x);
                    }
                }

                dp.push(prev[k].clone());
            }

            dp
        };

        let a = search(&nums);
        let reversed_nums: Vec<i32> = nums.iter().rev().map(|x| x.clone()).collect();
        let b = search(&reversed_nums);

        let mut result = 0;

        for i in k - 1..length - k {
            for &a_val in a[i].iter() {
                for &b_val in b[length - i - 2].iter() {
                    result = result.max(a_val ^ b_val);
                }
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
    fn test_3287() {
        assert_eq!(5, Solution::max_value(vec![2, 6, 7], 1));
        assert_eq!(2, Solution::max_value(vec![4, 2, 5, 6, 7], 2));
    }
}
