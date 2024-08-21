/**
 * [3007] Maximum Number That Sum of the Prices Is Less Than or Equal to K
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn find_maximum_number(k: i64, x: i32) -> i64 {
        let (mut left, mut right) = (1i64, (k + 1) << x);

        while left < right {
            let middle = (left + right + 1) / 2;
            
            if Self::accumulate_price(x, middle) > k {
                right = middle - 1;
            } else {
                left = middle;
            }
        }
        
        left
    }
    
    fn accumulate_price(x: i32, num: i64) -> i64 {
        let mut result = 0;
        let length = 64 - i64::leading_zeros(num) as i32;
        
        for i in (x..=length).step_by(x as usize) {
            result += Self::accumulate_bit_price(i, num);
        }
        
        result
    }
    
    fn accumulate_bit_price(x: i32, num: i64) -> i64 {
        let period = 1i64 << x;
        let mut result = period / 2 * (num / period);
        
        if num % period >= period / 2 {
            result += num % period - (period / 2 - 1);
        }
        
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3007() {
        assert_eq!(6, Solution::find_maximum_number(9, 1));
        assert_eq!(9, Solution::find_maximum_number(7, 2));
    }
}
