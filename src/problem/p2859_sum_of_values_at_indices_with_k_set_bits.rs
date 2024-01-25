/**
 * [2859] Sum of Values at Indices With K Set Bits
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut result = 0;

        for i in 0..nums.len() {
            if Solution::num_to_binary(i) == k {
                result += nums[i];
            }
        }

        result
    }

    fn num_to_binary(num: usize) -> usize {
        let mut bits = Vec::new();
        let mut num = num;

        while num != 0 {
            bits.push(num % 2);
            num = num / 2;
        }

        bits.iter().filter(|x| **x == 1).count()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2859() {
        assert_eq!(Solution::sum_indices_with_k_set_bits(vec![5,10,1,5,2], 1), 13);
        assert_eq!(Solution::sum_indices_with_k_set_bits(vec![4,3,2,1], 2), 1);
        assert_eq!(Solution::sum_indices_with_k_set_bits(vec![1], 0), 1);
        assert_eq!(Solution::sum_indices_with_k_set_bits(vec![1,1,3,1,6], 2), 1);
    }
}
