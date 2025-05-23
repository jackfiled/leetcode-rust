/**
 * [3068] Find the Maximum Sum of Node Values
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn maximum_value_sum(nums: Vec<i32>, k: i32, _: Vec<Vec<i32>>) -> i64 {
        let mut result: i64 = nums.iter().map(|x| *x as i64).sum();

        let mut difference: Vec<i64> = nums.iter().map(|&x| ((x ^ k) - x) as i64).collect();
        difference.sort_unstable_by(|a, b| b.cmp(a));

        for pair in difference.chunks(2) {
            let sum: i64 = pair.iter().sum();
            if pair.len() != 2 || sum < 0 {
                break;
            }

            result += sum;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3068() {
        assert_eq!(
            6,
            Solution::maximum_value_sum(vec![1, 2, 1], 3, vec![vec![0, 1], vec![0, 2]])
        );
        assert_eq!(
            9,
            Solution::maximum_value_sum(vec![2, 3], 7, vec![vec![0, 1]])
        );
        assert_eq!(
            42,
            Solution::maximum_value_sum(
                vec![7; 6],
                3,
                vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![0, 5]]
            )
        );
    }
}
