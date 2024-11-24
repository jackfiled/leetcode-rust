/**
 * [632] Smallest Range Covering Elements from K Lists
 */
pub struct Solution {}

// submission codes start here
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::rc::Rc;

#[derive(Eq, PartialEq)]
struct ArrayPointer<'a> {
    nums: &'a Vec<Vec<i32>>,
    pointers: Rc<RefCell<Vec<usize>>>,
    index: usize,
}

impl<'a> ArrayPointer<'a> {
    fn new(nums: &'a Vec<Vec<i32>>, pointers: Rc<RefCell<Vec<usize>>>, index: usize) -> Self {
        Self {
            nums,
            pointers,
            index,
        }
    }

    fn value(&self) -> i32 {
        self.nums[self.index][self.pointers.borrow()[self.index]]
    }
}

impl Ord for ArrayPointer<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        let left = self.nums[self.index][self.pointers.borrow()[self.index]];
        let right = self.nums[other.index][self.pointers.borrow()[other.index]];

        match left.cmp(&right) {
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
            Ordering::Greater => Ordering::Less,
        }
    }
}

impl PartialOrd for ArrayPointer<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let n = nums.len();
        let mut pointers = Rc::new(RefCell::new(vec![0; n]));
        let mut max = i32::MIN;

        let mut heap = BinaryHeap::new();
        for i in 0..n {
            heap.push(ArrayPointer::new(&nums, pointers.clone(), i));
            max = max.max(nums[i][0]);
        }

        let (mut left, mut right) = (0, i32::MAX);
        let mut min_range = right - left;

        loop {
            let min_index = heap.pop().unwrap();
            let current_range = max - min_index.value();
            if current_range < min_range {
                min_range = current_range;
                left = min_index.value();
                right = max;
            }

            pointers.borrow_mut()[min_index.index] += 1;
            if pointers.borrow()[min_index.index] == nums[min_index.index].len() {
                break;
            }

            max = max.max(min_index.value());
            heap.push(min_index);
        }

        vec![left, right]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_632() {
        assert_eq!(
            vec![20, 24],
            Solution::smallest_range(vec![
                vec![4, 10, 15, 24, 26],
                vec![0, 9, 12, 20],
                vec![5, 18, 22, 30]
            ])
        );

        assert_eq!(
            vec![1, 1],
            Solution::smallest_range(vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3],])
        );
    }
}
