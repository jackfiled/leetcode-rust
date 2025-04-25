use std::thread::current;

/**
 * [1287] Element Appearing More Than 25% In Sorted Array
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let mut last = arr[0];
        let mut count = 1;
        let threshold = arr.len() / 4;

        for &i in arr[1..].iter() {
            if last == i {
                count += 1;
            } else {
                last = i;
                count = 1;
            }

            if count > threshold {
                return i;
            }
        }

        last
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1287() {
        assert_eq!(
            6,
            Solution::find_special_integer(vec![1, 2, 2, 6, 6, 6, 6, 7, 10])
        );
        assert_eq!(1, Solution::find_special_integer(vec![1, 1]));
    }
}
