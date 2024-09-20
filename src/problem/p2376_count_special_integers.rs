
/**
 * [2376] Count Special Integers
 */
pub struct Solution {}


// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn count_special_numbers(n: i32) -> i32 {

        let n: Vec<i32> = n.to_string().chars().map(|c| c.to_digit(10).unwrap() as i32).collect();

        let mut result = 0;
        let mut candidate = 9;

        // 计算位数小于n的特殊整数
        for i in 0..n.len() as i32 - 1 {
            result += candidate;
            candidate *= 9 - i
        }

        let mut memory: HashMap<i32, i32> = HashMap::new();
        
        result += Self::dp(0, false, &mut memory, &n);

        result
    }
   
    // mask 表示前缀中使用过的数字 使用二进制表示
    // prefix_smaller 当前的前缀是否小于n的前缀
    fn dp(mask: i32, prefix_smaller: bool, memory: &mut HashMap<i32, i32>, number: &Vec<i32>) -> i32 {
        let used_bits = mask.count_ones() as usize;
        if used_bits == number.len() {
            return 1;
        }

        let key = mask * 2 + if prefix_smaller {
            1
        } else {
            0
        };
        
        if !memory.contains_key(&key) {
            let mut result = 0;

            let lower_bound = if mask == 0 {
                1
            } else {
                0
            };

            let upper_bound = if prefix_smaller {
                9
            } else {
                number[used_bits]
            };

            for i in lower_bound..=upper_bound {
                if mask >> i & 1 == 0 {
                    result += Self::dp(mask | 1 << i, prefix_smaller || i < upper_bound, memory, number);
                }
            }

            memory.insert(key, result);
        }

        *memory.get(&key).unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2376() {
        assert_eq!(19, Solution::count_special_numbers(20));
        assert_eq!(5, Solution::count_special_numbers(5));
        assert_eq!(110, Solution::count_special_numbers(135));
    }
}
