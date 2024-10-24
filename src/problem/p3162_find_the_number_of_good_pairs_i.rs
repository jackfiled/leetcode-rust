/**
 * [3162] Find the Number of Good Pairs I
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i32 {
        let nums1: Vec<i32> = nums1
            .into_iter()
            .filter_map(|x| if x % k == 0 { Some(x / k) } else { None })
            .collect();

        let mut result = 0;

        for &num in nums1.iter() {
            for &num2 in nums2.iter() {
                if num % num2 == 0 {
                    result += 1;
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
    fn test_3162() {
        assert_eq!(
            5,
            Solution::number_of_pairs(vec![1, 3, 4], vec![1, 3, 4], 1)
        );
        assert_eq!(
            2,
            Solution::number_of_pairs(vec![1, 2, 4, 12], vec![2, 4], 3)
        );
        assert_eq!(1, Solution::number_of_pairs(vec![2, 12], vec![4, 3], 4));
    }
}
