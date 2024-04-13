/**
 * [274] H-Index
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut h_array = vec![0; 1001];

        for i in citations {
            let i = i as usize;

            h_array[i] += 1;
        }

        let mut count = 0;
        for i in (0..=1000).rev() {
            count += h_array[i];

            if count >= i {
                return i as i32;
            }
        }

        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_274() {
        assert_eq!(3, Solution::h_index(vec![3, 0, 6, 1, 5]));
        assert_eq!(1, Solution::h_index(vec![1,3,1]));
    }
}
