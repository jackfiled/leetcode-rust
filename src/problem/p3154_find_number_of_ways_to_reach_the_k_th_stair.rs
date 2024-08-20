/**
 * [3154] Find Number of Ways to Reach the K-th Stair
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn ways_to_reach_stair(k: i32) -> i32 {
        let mut n = 0;
        let mut pos = 1;
        let mut result = 0;

        loop {
            if pos - n - 1 <= k && k <= pos {
                result += Self::combine(n + 1, pos - k);
            }
            
            if pos - n - 1 > k {
                break;
            }
            
            n += 1;
            pos *= 2;
        }
        
        result
    }

    /// C_n^k = \frac{n * (n - 1) * .. * (n - k + 1)}{1 * 2 * 3 * ... * k}
    fn combine(n : i32, k: i32) -> i32 {
        let mut result: i64 = 1;
        let (n, k) = (n as i64, k as i64);
        
        for i in (n - k + 1..=n).rev() {
            result *= i;
            result /= n - i  + 1;
        }
        
        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3154() {
        assert_eq!(2, Solution::ways_to_reach_stair(0));
        assert_eq!(4, Solution::ways_to_reach_stair(1));
    }
}
