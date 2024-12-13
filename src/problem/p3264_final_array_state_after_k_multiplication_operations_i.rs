/**
 * [3264] Final Array State After K Multiplication Operations I
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn get_final_state(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        let mut nums = nums;

        for _ in 0..k {
            let min_value = nums.iter_mut().min().unwrap();
            *min_value = *min_value * multiplier;
        }

        nums
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3264() {
        assert_eq!(
            vec![8, 4, 6, 5, 6],
            Solution::get_final_state(vec![2, 1, 3, 5, 6], 5, 2)
        );
        assert_eq!(vec![16, 8], Solution::get_final_state(vec![1, 2], 3, 4));
    }
}
