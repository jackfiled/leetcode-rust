/**
 * [2552] Count Increasing Quadruplets
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn count_quadruplets(nums: Vec<i32>) -> i64 {
        let nums: Vec<usize> = nums.iter().map(|x| *x as usize).collect();
        let n = nums.len();
        let mut pre = vec![0;n + 1];
        let mut result = 0;
        
        for j in 0..n {
            let mut suffix = 0;
            
            for k in (j + 1..n).rev() {
                if nums[j] > nums[k] {
                    result += pre[nums[k]] * suffix;
                } else { 
                    suffix += 1;
                }
            }
            
            for i in nums[j] + 1..=n {
                pre[i] += 1;
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
    fn test_2552() {
        assert_eq!(2, Solution::count_quadruplets(vec![1,3,2,4,5]));
    }
}
