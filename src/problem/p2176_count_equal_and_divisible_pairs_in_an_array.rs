/**
 * [2176] Count Equal and Divisible Pairs in an Array
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let n = nums.len();

        (0..n)
            .into_iter()
            .map(|x| (x + 1..n).into_iter().map(move |y| (x, y)))
            .flatten()
            .filter_map(|(i, j)| {
                if nums[i] == nums[j] && i * j % k == 0 {
                    Some(())
                } else {
                    None
                }
            })
            .count() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2176() {
        assert_eq!(4, Solution::count_pairs(vec![3, 1, 2, 2, 2, 1, 3], 2));
        assert_eq!(0, Solution::count_pairs(vec![1, 2, 3, 4], 1));
    }
}
