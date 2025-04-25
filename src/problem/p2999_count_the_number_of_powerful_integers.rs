/**
 * [2999] Count the Number of Powerful Integers
 */
pub struct Solution {}

// submission codes start here
use std::str::FromStr;

struct Searcher {
    low: Vec<u8>,
    high: Vec<u8>,
    suffix: Vec<u8>,
    memory: Vec<i64>,
    limit: u8,
}

impl Searcher {
    fn new(start: i64, finish: i64, limit: i32, s: String) -> Self {
        let mut low: Vec<u8> = start.to_string().bytes().map(|x| x - b'0').collect();
        let high: Vec<u8> = finish.to_string().bytes().map(|x| x - b'0').collect();

        // 对齐low和high的数位
        for _ in 0..high.len() - low.len() {
            low.insert(0, 0);
        }

        let suffix = s.bytes().map(|x| x - b'0').collect();
        let n = high.len();

        Self {
            low,
            high,
            suffix,
            memory: vec![-1; n],
            limit: limit as u8,
        }
    }

    fn search(&mut self, i: usize, limit_low: bool, limit_high: bool) -> i64 {
        if i == self.low.len() {
            return 1;
        }

        if !limit_low && !limit_high && self.memory[i] != -1 {
            return self.memory[i];
        }

        let low = if limit_low { self.low[i] } else { 0 };

        let high = if limit_high { self.high[i] } else { 9 };

        let mut result = 0;

        let prefix_len = self.low.len() - self.suffix.len();
        if i < prefix_len {
            for digit in low..=high.min(self.limit) {
                result += self.search(
                    i + 1,
                    limit_low && digit == low,
                    limit_high && digit == high,
                );
            }
        } else {
            let digit = self.suffix[i - prefix_len];
            if digit >= low && digit <= high.min(self.limit) {
                result = self.search(
                    i + 1,
                    limit_low && digit == low,
                    limit_high && digit == high,
                );
            }
        }

        if !limit_low && !limit_high {
            self.memory[i] = result;
        }

        result
    }
}

impl Solution {
    pub fn number_of_powerful_int(start: i64, finish: i64, limit: i32, s: String) -> i64 {
        let mut searcher = Searcher::new(start, finish, limit, s);

        searcher.search(0, true, true)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2999() {
        assert_eq!(
            5,
            Solution::number_of_powerful_int(1, 6000, 4, "123".to_owned())
        );
        assert_eq!(
            2,
            Solution::number_of_powerful_int(15, 215, 6, "10".to_owned())
        );
        assert_eq!(
            0,
            Solution::number_of_powerful_int(1000, 2000, 4, "3000".to_owned())
        );
    }
}
