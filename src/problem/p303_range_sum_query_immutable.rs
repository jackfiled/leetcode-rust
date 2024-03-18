/**
 * [303] Range Sum Query - Immutable
 */
pub struct Solution {}

// submission codes start here

struct NumArray {
    prefix_array: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut prefix_array = Vec::with_capacity(nums.len());

        for (index, value) in nums.iter().enumerate() {
            if index == 0 {
                prefix_array.push(*value);
            } else {
                prefix_array.push(prefix_array[index - 1] + *value);
            }
        }

        NumArray { prefix_array }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let left = left as usize;
        let right = right as usize;

        return if left == 0 {
            self.prefix_array[right]
        } else {
            self.prefix_array[right] - self.prefix_array[left - 1]
        };
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * let ret_1: i32 = obj.sum_range(left, right);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_303() {}
}
