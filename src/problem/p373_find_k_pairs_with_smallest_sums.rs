/**
 * [373] Find K Pairs with Smallest Sums
 */
pub struct Solution {}


// submission codes start here
use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
struct Pair<'a> {
    nums1: &'a Vec<i32>,
    nums2: &'a Vec<i32>,
    x: usize,
    y: usize,
}

impl Pair<'_> {
    fn new<'a>(nums1: &'a Vec<i32>, nums2: &'a Vec<i32>, x: usize, y: usize) -> Pair<'a> {
        Pair {
            nums1,
            nums2,
            x,
            y,
        }
    }
}

impl Ord for Pair<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.nums1[other.x] + other.nums2[other.y]).cmp(
            &(self.nums1[self.x] + self.nums2[self.y]))
    }
}

impl PartialOrd for Pair<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        use std::collections::BinaryHeap;

        let (m, n) = (nums1.len(), nums2.len());
        let k = k as usize;

        let mut heap = BinaryHeap::new();

        for i in 0..m.min(k) {
            heap.push(Pair::new(&nums1, &nums2, i, 0));
        }

        let mut count = 0;
        let mut result = Vec::with_capacity(k);

        while count < k && !heap.is_empty() {
            let pair = heap.pop().unwrap();
            let (x, y) = (pair.x, pair.y);
            result.push((nums1[x], nums2[y]));

            if y + 1 < n {
                heap.push(Pair::new(&nums1, &nums2, x, y + 1));
            }

            count += 1;
        }


        result.iter().map(|(x, y)| vec![*x, *y]).collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_373() {
        assert_eq!(vec![vec![1, 2], vec![1, 4], vec![1, 6]],
                   Solution::k_smallest_pairs(vec![1, 7, 11], vec![2, 4, 6], 3));
    }
}
