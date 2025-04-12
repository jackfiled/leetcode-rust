/**
 * [3272] Find the Count of Good Integers
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashSet;

impl Solution {
    pub fn count_good_integers(n: i32, k: i32) -> i64 {
        let n = n as usize;
        let k = k as i64;
        let base_value = 10i64.pow((n as u32 - 1) / 2);
        let length = (n + 1) / 2;
        let mut dict = HashSet::new();

        // 枚举n个数位的回文数
        for mut i in base_value..base_value * 10 {
            let mut s = Vec::with_capacity(length);

            while i != 0 {
                s.push(i % 10);
                i = i / 10;
            }

            // 这里拼接回文字符串有点困难
            // 因为上面存数字字符存的是倒序
            // 即 123 -> 3 2 1
            // 所以 12321 -> 1 2 3 2 1
            let mut real_str: Vec<i64> = s
                .iter()
                .rev()
                .chain(s.iter().skip(n & 1))
                .map(|x| *x)
                .collect();

            let number = real_str.iter().fold(0, |v, i| v * 10 + *i);
            if number % k == 0 {
                real_str.sort_unstable();
                dict.insert(real_str);
            }
        }

        let mut factorial = Vec::with_capacity(n + 1);
        factorial.push(1);
        for i in 1..=n {
            factorial.push(factorial[i - 1] * i as i64);
        }

        let mut result = 0;

        for str in dict.into_iter() {
            let mut counts = [0; 10];
            for i in str {
                counts[i as usize] += 1;
            }

            let mut total = (n - counts[0]) as i64 * factorial[n - 1];
            for x in counts {
                total /= factorial[x]
            }
            result += total;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3272() {
        assert_eq!(27, Solution::count_good_integers(3, 5));
        assert_eq!(2, Solution::count_good_integers(1, 4));
        assert_eq!(2468, Solution::count_good_integers(5, 6));
    }
}
