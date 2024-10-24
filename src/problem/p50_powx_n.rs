/**
 * [50] Pow(x, n)
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n < 0 {
            1f64 / Self::quick_pow(x, -n)
        } else {
            Self::quick_pow(x, n)
        }
    }

    fn quick_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            1f64
        } else {
            let y = Self::quick_pow(x, n / 2);
            if n % 2 == 0 {
                y * y
            } else {
                y * y * x
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_50() {}
}
