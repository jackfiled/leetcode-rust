/**
 * [75] Sort Colors
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut zero_count, mut one_count, mut two_count) = (0, 0, 0);

        for i in nums.iter() {
            match i {
                &0 => zero_count += 1,
                &1 => one_count += 1,
                &2 => two_count += 1,
                _ => {}
            }
        }

        let mut pos = 0;
        for _ in 0..zero_count {
            nums[pos] = 0;
            pos += 1;
        }
        for _ in 0..one_count {
            nums[pos] = 1;
            pos += 1;
        }
        for _ in 0..two_count {
            nums[pos] = 2;
            pos += 1;
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_75() {
        let mut input = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut input);
        assert_eq!(vec![0, 0, 1, 1, 2, 2], input);

        let mut input = vec![2, 0, 1];
        Solution::sort_colors(&mut input);
        assert_eq!(vec![0, 1, 2], input);
    }
}
