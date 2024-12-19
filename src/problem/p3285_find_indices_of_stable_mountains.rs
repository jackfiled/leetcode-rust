/**
 * [3285] Find Indices of Stable Mountains
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn stable_mountains(height: Vec<i32>, threshold: i32) -> Vec<i32> {
        (1..height.len())
            .filter_map(|i| {
                if height[i - 1] > threshold {
                    Some(i as i32)
                } else {
                    None
                }
            })
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3285() {
        assert_eq!(
            vec![3, 4],
            Solution::stable_mountains(vec![1, 2, 3, 4, 5], 2)
        );
        assert_eq!(
            Vec::<i32>::new(),
            Solution::stable_mountains(vec![10, 1, 10, 1, 10], 10)
        );
    }
}
