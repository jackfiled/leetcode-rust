/**
 * [2864] Maximum Odd Binary Number
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        let nums: Vec<u32> = s.chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        let mut one_count = 0;
        let mut zero_count = 0;

        for num in &nums {
            if *num == 1 {
                one_count += 1;
            } else if *num == 0 {
                zero_count += 1;
            }
        }

        let mut result: Vec<char> = Vec::with_capacity(nums.len());

        for _ in  1..one_count {
            result.push('1');
        }

        for _ in 0..zero_count {
            result.push('0');
        }

        result.push('1');
        result.iter().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2864() {
        assert_eq!("001", Solution::maximum_odd_binary_number("010".to_owned()));
        assert_eq!("1001", Solution::maximum_odd_binary_number("0101".to_owned()));
    }
}
