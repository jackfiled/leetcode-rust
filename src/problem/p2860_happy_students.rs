/**
 * [2860] Happy Students
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn count_ways(nums: Vec<i32>) -> i32 {
        let mut nums: Vec<usize> = nums.iter().map(|x| *x as usize).collect();
        let n = nums.len();
        nums.sort();

        // 是否能不选中任何学生
        let mut result = 0; 

        for i in 0..=n {
            if i > 0 && nums[i - 1] >= i {
                continue;
            } 
            
            if i < n && nums[i] <= i {
                continue;
            }
            
            result += 1;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2860() {
        assert_eq!(2, Solution::count_ways(vec![1, 1]));
        assert_eq!(3, Solution::count_ways(vec![6, 0, 3, 3, 6, 7, 2, 7]));
        assert_eq!(1, Solution::count_ways(vec![1, 0, 1, 1]));
    }
}
