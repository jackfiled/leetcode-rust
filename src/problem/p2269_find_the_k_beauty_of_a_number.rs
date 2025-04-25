/**
 * [2269] Find the K-Beauty of a Number
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn divisor_substrings(num: i32, k: i32) -> i32 {
        let k = k as usize;
        let num_str: Vec<char> = num.to_string().chars().collect();

        (0..=num_str.len() - k)
            .into_iter()
            .map(|i| {
                (&num_str[i..i + k])
                    .iter()
                    .fold(0i32, |v, c| v * 10 + c.to_digit(10).unwrap() as i32)
            })
            .filter(|&v| v != 0 && num % v == 0)
            .count() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2269() {
        assert_eq!(2, Solution::divisor_substrings(240, 2));
        assert_eq!(2, Solution::divisor_substrings(430043, 2));
    }
}
