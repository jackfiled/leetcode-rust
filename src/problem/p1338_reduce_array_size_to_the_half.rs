/**
 * [1338] Reduce Array Size to The Half
 */
pub struct Solution {}

// submission codes start here
use std::collections::BinaryHeap;
use std::collections::HashMap;

impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let mut map = HashMap::new();

        for &i in arr.iter() {
            let entry = map.entry(i).or_insert(0);
            *entry += 1;
        }

        let mut heap = BinaryHeap::with_capacity(map.len());

        for (_, v) in map {
            heap.push(v);
        }

        let mut result = 0;
        let mut count = arr.len();
        let target = count / 2;

        while count > target {
            if let Some(head) = heap.pop() {
                count -= head;
                result += 1;
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1338() {
        assert_eq!(
            2,
            Solution::min_set_size(vec![3, 3, 3, 3, 5, 5, 5, 2, 2, 7])
        );
        assert_eq!(1, Solution::min_set_size(vec![7, 7, 7, 7, 7, 7]));
    }
}
