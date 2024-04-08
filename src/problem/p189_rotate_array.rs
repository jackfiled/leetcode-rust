/**
 * [189] Rotate Array
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = (k as usize) % nums.len();

        if k == 0 {
            return;
        }

        let mut temp = Vec::with_capacity(k);

        for i in (nums.len() - k)..nums.len() {
            temp.push(nums[i]);
        }

        for i in (0..(nums.len() - k)).rev() {
            nums[i + k] = nums[i];
        }

        for i in 0..k {
            nums[i] = temp[i];
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_189() {
        let mut array = vec![1, 2, 3, 4, 5, 6, 7];
        Solution::rotate(&mut array, 3);
        assert_eq!(vec![5, 6, 7, 1, 2, 3, 4], array);
    }
}
