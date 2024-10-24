/**
 * [2024] Maximize the Confusion of an Exam
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
        let str: Vec<char> = answer_key.chars().collect();

        Self::max_consecutive_char(&str, k, 'T').max(Self::max_consecutive_char(&str, k, 'F'))
    }

    fn max_consecutive_char(str: &Vec<char>, k: i32, c: char) -> i32 {
        let n = str.len();
        let mut result = 0;

        let (mut left, mut sum) = (0, 0);
        for right in 0..n {
            sum += match str[right] == c {
                true => 0,
                false => 1,
            };

            while sum > k {
                sum -= match str[left] == c {
                    true => 0,
                    false => 1,
                };
                left += 1;
            }

            result = result.max(right - left + 1);
        }

        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2024() {
        assert_eq!(4, Solution::max_consecutive_answers("TTFF".to_owned(), 2));
        assert_eq!(3, Solution::max_consecutive_answers("TFFT".to_owned(), 1));
    }
}
