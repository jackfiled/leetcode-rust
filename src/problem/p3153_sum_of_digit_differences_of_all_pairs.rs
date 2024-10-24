/**
 * [3153] Sum of Digit Differences of All Pairs
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn sum_digit_differences(nums: Vec<i32>) -> i64 {
        let n = nums.len() as i64;
        let mut result = 0;

        let mut base = 1;
        loop {
            if nums[0] / base == 0 {
                break;
            }

            let mut map = vec![0; 10];
            for &num in nums.iter() {
                let digit = (num / base % 10) as usize;
                map[digit] += 1;
            }

            result += map
                .iter()
                .filter_map(|x| if *x == 0 { None } else { Some(*x * (n - *x)) })
                .sum::<i64>()
                / 2;

            base *= 10;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3153() {
        assert_eq!(4, Solution::sum_digit_differences(vec![13, 23, 12]));
        assert_eq!(0, Solution::sum_digit_differences(vec![10, 10, 10, 10]));
    }
}
