/**
 * [3258] Count Substrings That Satisfy K-Constraint I
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn count_k_constraint_substrings(s: String, k: i32) -> i32 {
        let s: Vec<u32> = s.chars().map(|x| x.to_digit(10).unwrap()).collect();

        let mut result = 0;
        let mut windows = Vec::with_capacity(s.len());

        let mut window = (0, 0);
        for i in 0..s.len() {
            if s[i] == 0 {
                window.0 += 1;
            } else {
                window.1 += 1;
            }

            windows.push(window);
            if window.0 <= k || window.1 <= k {
                result += 1;
            }
        }

        for i in 1..s.len() {
            for length in 1..=s.len() {
                if i >= length {
                    if s[i] == 0 {
                        windows[length - 1].0 += 1;
                    } else {
                        windows[length - 1].1 += 1;
                    }

                    if s[i - length] == 0 {
                        windows[length - 1].0 -= 1;
                    } else {
                        windows[length - 1].1 -= 1;
                    }

                    if windows[length - 1].0 <= k || windows[length - 1].1 <= k {
                        result += 1;
                    }
                }
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3258() {
        assert_eq!(
            12,
            Solution::count_k_constraint_substrings("10101".to_owned(), 1)
        );
        assert_eq!(
            25,
            Solution::count_k_constraint_substrings("1010101".to_owned(), 2)
        );
        assert_eq!(
            15,
            Solution::count_k_constraint_substrings("11111".to_owned(), 1)
        );
    }
}
