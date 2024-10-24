/**
 * [977] Squares of a Sorted Array
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::with_capacity(nums.len());
        let mut result = Vec::with_capacity(nums.len());

        for i in nums {
            if i < 0 {
                stack.push(i);
                continue;
            }

            // 弹出当前栈中小于当前数字的数字
            while let Some(&tail) = stack.last() {
                let tail = -tail;
                if tail > i {
                    break;
                }

                result.push(tail.pow(2));
                stack.pop();
            }

            result.push(i.pow(2));
        }

        while let Some(tail) = stack.pop() {
            result.push(tail.pow(2));
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_977() {
        assert_eq!(
            vec![0, 1, 9, 16, 100],
            Solution::sorted_squares(vec![-4, -1, 0, 3, 10])
        );
        assert_eq!(
            vec![4, 9, 9, 49, 121],
            Solution::sorted_squares(vec![-7, -3, 2, 3, 11])
        );
        assert_eq!(vec![1, 4], Solution::sorted_squares(vec![-2, -1]));
    }
}
