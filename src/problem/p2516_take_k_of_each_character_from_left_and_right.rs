/**
 * [2516] Take K of Each Character From Left and Right
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn take_characters(s: String, k: i32) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let n = s.len();

        let mut left_counts = vec![0; 3];

        for &c in s.iter() {
            match c {
                'a' => left_counts[0] += 1,
                'b' => left_counts[1] += 1,
                'c' => left_counts[2] += 1,
                _ => unreachable!(),
            }       
        }

        if left_counts.iter().any(|x| *x < k) {
            return -1;
        }
        
        let mut result = 0;
        let mut right = 0;
        let mut right_counts = vec![0;3];
        
        // 左指针从右往左移动
        for left in (0..n - 1).rev() {
            
        }
        
        
        

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2516() {
    }
}
