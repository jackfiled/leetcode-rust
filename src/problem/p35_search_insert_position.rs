/**
 * [35] Search Insert Position
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;
        let mut result = nums.len();

        while l <= r {
            let middle = (r - l) / 2 + l;

            if target <= nums[middle] {
                result = middle;
                if middle == 0 {
                    break;
                }
                r = middle - 1;
            } else {
                l = middle + 1;
            }
        }

        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_35() {
    }
}
