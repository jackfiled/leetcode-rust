/**
 * [27] Remove Element
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        let mut last = nums.len() - 1;
        let mut i = 0;

        while i <= last {
            while nums[i] == val {
                if i == last {
                    return last as i32;
                }

                nums[i] = nums[last];
                last -= 1;
            }

            i += 1;
        }

        (last + 1) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_27() {
        let mut array = vec![3, 2, 2, 3];
        assert_eq!(2, Solution::remove_element(&mut array, 3));
        for i in 0..2 {
            assert_eq!(2, array[i]);
        }

        let mut array = vec![3];
        assert_eq!(0, Solution::remove_element(&mut array, 3));
    }
}
