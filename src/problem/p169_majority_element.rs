/**
 * [169] Majority Element
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut candidate = nums[0];

        for i in nums {
            if count == 0 {
                candidate = i;
            }

            count += if candidate == i {
                1
            } else {
                -1
            };
        }

        candidate
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_169() {
        assert_eq!(3, Solution::majority_element(vec![3,2,3]));
    }
}
