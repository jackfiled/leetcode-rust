pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;

        for i in nums {
            if i < k {
                result += 1;
            }
        }

        result
    }
}