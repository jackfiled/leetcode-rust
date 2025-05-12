/**
 * [2094] Finding 3-Digit Even Numbers
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashSet;

impl Solution {
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        let mut result = HashSet::new();

        for (x, first) in
            digits
                .iter()
                .enumerate()
                .filter_map(|(i, &x)| if x != 0 { Some((i, x)) } else { None })
        {
            for (y, second) in
                digits
                    .iter()
                    .enumerate()
                    .filter_map(|(i, &y)| if i != x { Some((i, y)) } else { None })
            {
                for third in digits.iter().enumerate().filter_map(|(i, &z)| {
                    if i != x && i != y && z % 2 == 0 {
                        Some(z)
                    } else {
                        None
                    }
                }) {
                    result.insert(first * 100 + second * 10 + third);
                }
            }
        }

        let mut result: Vec<i32> = result.into_iter().collect();
        result.sort_unstable();
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2094() {
        assert_eq!(
            vec![102, 120, 130, 132, 210, 230, 302, 310, 312, 320],
            Solution::find_even_numbers(vec![2, 1, 3, 0])
        );
        assert_eq!(
            vec![222, 228, 282, 288, 822, 828, 882],
            Solution::find_even_numbers(vec![2, 2, 8, 8, 2])
        );
        assert_eq!(
            Vec::<i32>::new(),
            Solution::find_even_numbers(vec![3, 7, 5])
        );
    }
}
