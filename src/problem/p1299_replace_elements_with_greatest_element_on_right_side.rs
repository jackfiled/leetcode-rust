/**
 * [1299] Replace Elements with Greatest Element on Right Side
 */
pub struct Solution {}

// submission codes start here
use std::collections::BinaryHeap;
use std::iter::FromIterator;

impl Solution {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let mut heap: BinaryHeap<(i32, usize)> =
            BinaryHeap::from_iter(arr[1..].into_iter().enumerate().map(|(i, v)| (*v, i + 1)));

        let mut result = Vec::with_capacity(arr.len());

        for i in 0..arr.len() - 1 {
            while let Some(head) = heap.peek() {
                if head.1 > i {
                    break;
                }

                heap.pop();
            }
            if let Some(head) = heap.peek() {
                result.push(head.0);
            }
        }

        result.push(-1);
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1299() {
        assert_eq!(
            vec![18, 6, 6, 6, 1, -1],
            Solution::replace_elements(vec![17, 18, 5, 4, 6, 1])
        );
        assert_eq!(vec![-1], Solution::replace_elements(vec![400]));
    }
}
