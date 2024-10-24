/**
 * [69] Sqrt(x)
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        // 牛顿迭代法
        if x == 0 {
            return 0;
        }

        let c = x as f64;
        let mut x = x as f64;

        loop {
            let xi = 0.5 * (x + c / x);
            if (x - xi).abs() < 1e-7 {
                break;
            }
            x = xi;
        }

        return x as i32;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_69() {
        assert_eq!(0, Solution::my_sqrt(0));
        assert_eq!(1, Solution::my_sqrt(1));
        assert_eq!(1, Solution::my_sqrt(2));
        assert_eq!(2, Solution::my_sqrt(4));
        assert_eq!(2, Solution::my_sqrt(8));
    }
}
