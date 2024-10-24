/**
 * [2719] Count of Integers
 */
pub struct Solution {}

// submission codes start here

struct DigitDP {
    num1: Vec<u32>,
    num2: Vec<u32>,
    min_sum: u32,
    max_sum: u32,

    dp_vec: Vec<Vec<i32>>,
}

impl DigitDP {
    pub fn new(num1: Vec<u32>, num2: Vec<u32>, min_sum: u32, max_sum: u32) -> DigitDP {
        let mut dp = Vec::with_capacity(num2.len());

        for _ in 0..num2.len() {
            let length = num2.len() * 9 + 1;
            let mut array = Vec::with_capacity(length);
            array.resize(length, -1);

            dp.push(array)
        }

        DigitDP {
            num1,
            num2,
            min_sum,
            max_sum,
            dp_vec: dp,
        }
    }

    pub fn dp(&mut self, sum: u32, l_limit: bool, r_limit: bool, index: usize) -> i32 {
        if index >= self.num2.len() {
            return if sum >= self.min_sum && sum <= self.max_sum {
                1
            } else {
                0
            };
        }

        if !l_limit && !r_limit && self.dp_vec[index][sum as usize] != -1 {
            return self.dp_vec[index][sum as usize];
        }

        let low = if l_limit { self.num1[index] } else { 0 };

        let high = if r_limit { self.num2[index] } else { 9 };

        let mut result = 0;
        for i in low..=high {
            result += DigitDP::dp(
                self,
                sum + i,
                i == self.num1[index] && l_limit,
                i == self.num2[index] && r_limit,
                index + 1,
            );

            if result > 1000000007 {
                result = result % 1000000007
            }
        }

        if !l_limit && !r_limit {
            self.dp_vec[index][sum as usize] = result;
        }

        result
    }
}

impl Solution {
    pub fn count(num1: String, num2: String, min_sum: i32, max_sum: i32) -> i32 {
        let mut num1 = Solution::str_to_vec(num1);
        let num2 = Solution::str_to_vec(num2);

        // 将两个数组的长度对齐，在num1前面填0
        for _ in 0..(num2.len() - num1.len()) {
            num1.insert(0, 0);
        }

        let mut digit_dp = DigitDP::new(num1, num2, min_sum as u32, max_sum as u32);

        digit_dp.dp(0, true, true, 0)
    }

    fn str_to_vec(num: String) -> Vec<u32> {
        let mut num_vec = Vec::new();

        for c in num.chars() {
            match c.to_digit(10) {
                None => {}
                Some(i) => num_vec.push(i),
            }
        }

        num_vec
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2719() {
        assert_eq!(
            Solution::count(String::from("1"), String::from("12"), 1, 8),
            11
        );

        assert_eq!(
            Solution::count(String::from("1"), String::from("5"), 1, 5),
            5
        );
    }
}
