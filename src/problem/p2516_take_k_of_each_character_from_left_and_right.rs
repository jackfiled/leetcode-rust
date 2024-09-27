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
        
        let mut result = n;
        let mut right = 0;
        
        // 左指针从右往左移动
        for left in (0..=n - 1).rev() {
            while left_counts.iter().any(|x| *x < k) {
                match s[n - 1 - right] {
                    'a' => left_counts[0] += 1,
                    'b' => left_counts[1] += 1,
                    'c' => left_counts[2] += 1,
                    _ => unreachable!()
                };

                right += 1;
            }

            result = result.min(left + 1 + right);
            
            match s[left] {
                'a' => left_counts[0] -= 1,
                'b' => left_counts[1] -= 1,
                'c' => left_counts[2] -= 1,
                _ => unreachable!()
            };
        }

        if !left_counts.iter().any(|x| *x < k) {
            result = result.min(right);
        }

        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2516() {
        assert_eq!(8, Solution::take_characters("aabaaaacaabc".to_owned(), 2));
        assert_eq!(-1, Solution::take_characters("a".to_owned(), 1));
        assert_eq!(0, Solution::take_characters("a".to_owned(), 0));
    }
}
