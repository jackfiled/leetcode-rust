/**
 * [2917] Find the K-or of an Array
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn find_k_or(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;

        for i in 0..32 {
            let mut count = 0;

            let x = 1 << i;
            for num in &nums {
                if *num & x == x {
                    count += 1;
                }
            }

            if count >= k {
                dbg!(x);
                result += x;
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
    fn test_2917() {
        assert_eq!(9, Solution::find_k_or(vec![7, 12, 9, 8, 9, 15], 4));
    }
}
