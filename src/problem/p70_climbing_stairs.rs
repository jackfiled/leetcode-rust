/**
 * [70] Climbing Stairs
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1 {
            return 1;
        } else if n == 2 {
            return 2;
        }
        
        let mut last_last = 1;
        let mut last = 2;
        
        for i in 3..=n {
            let now = last + last_last;
            last_last = last;
            last = now;
        }
        
        last
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_70() {
        assert_eq!(2, Solution::climb_stairs(2));
        assert_eq!(3, Solution::climb_stairs(3));
        assert_eq!(5, Solution::climb_stairs(4));
    }
}
