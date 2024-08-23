/**
 * [3145] Find Products of Elements of Big Array
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn find_products_of_elements(queries: Vec<Vec<i64>>) -> Vec<i32> {
        let mut result_array = Vec::with_capacity(queries.len());

        for query in queries {
            let (start, end) = (query[0] + 1, query[1] + 1);
            let left = Self::middle_check(start);
            let right = Self::middle_check(end);
            let mod_number = query[2];

            let mut result = 1;
            let mut pre = Self::count_one(left - 1);

            for i in 0..=60 {
                if (1 << i) & left != 0 {
                    pre += 1;

                    if pre >= start && pre <= end {
                        result = result * (1 << i) % mod_number;
                    }
                }
            }

            if right > left {
                let mut back = Self::count_one(right - 1);
                for i in 0..=60 {
                    if (1 << i) & right != 0 {
                        back += 1;

                        if back >= start && back <= end {
                            result = result * (1 << i) % mod_number;
                        }
                    }
                }
            }

            if right - left > 1 {
                let middle = Self::count_power(right - 1) - Self::count_power(left);

                result = result * Self::power_mod(2, middle, mod_number) % mod_number;
            }

            result_array.push(result);
        }

        result_array.iter().map(|x| *x as i32).collect()
    }

    fn middle_check(x: i64) -> i64 {
        let (mut left, mut right) = (1i64, 1e15 as i64);

        while left < right {
            let middle = (left + right) >> 1;
            if Self::count_one(middle) >= x {
                right = middle;
            } else {
                left = middle + 1;
            }
        }

        right
    }

    /// 计算小于等于x的数中数位1的和
    fn count_one(x: i64) -> i64 {
        let mut result = 0;
        let mut sum = 0;

        for i in (0..=60).rev() {
            if (1 << i) & x != 0 {
                result += sum * (1 << i);
                sum += 1;

                if i > 0 {
                    result += i * (1 << (i - 1));
                }
            }
        }

        result + sum
    }

    /// 计算小于等于x所有数的数位对幂的贡献之和 
    fn count_power(x: i64) -> i64 {
        let mut result = 0;
        let mut sum = 0;

        for i in (0..=60).rev() {
            if (1 << i) & x != 0 {
                result += sum * (1 << i);
                sum += i;

                if i > 0 {
                    result += i * (i - 1) / 2 * (1 << (i - 1));
                }
            }
        }

        result + sum
    }

    /// Calculate (x ^ y) % mod_number
    fn power_mod(x: i64, y: i64, mod_number: i64) -> i64 {
        let mut result = 1i64;
        let (mut x, mut y) = (x, y);

        while y != 0 {
            if y & 1 == 1 {
                result = result * x % mod_number;
            }

            x = x * x % mod_number;
            y = y >> 1;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3145() {
        assert_eq!(vec![4], Solution::find_products_of_elements(vec![vec![1, 3, 7]]));
        assert_eq!(vec![2, 2], Solution::find_products_of_elements(vec![vec![2, 5, 3], vec![7, 7, 4]]));
    }
}
