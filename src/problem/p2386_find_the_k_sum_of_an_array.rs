/**
 * [2386] Find the K-Sum of an Array
 */
pub struct Solution {}

// submission codes start here
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq)]
struct Node {
    sum: i64,
    last: usize,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.sum.partial_cmp(&self.sum)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.sum.cmp(&self.sum)
    }
}

impl Solution {
    pub fn k_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut total = 0;
        let mut nums: Vec<i64> = nums
            .iter()
            .map(|x| {
                if *x > 0 {
                    let x = *x as i64;
                    total += x;
                    x
                } else {
                    -x as i64
                }
            })
            .collect();

        nums.sort_unstable();

        let mut heap = BinaryHeap::new();
        heap.push(Node {
            sum: nums[0],
            last: 0,
        });

        let mut result = 0;
        for _ in 1..k {
            let top = heap.pop().unwrap();

            result = top.sum;
            if top.last == nums.len() - 1 {
                continue;
            }

            heap.push(Node {
                sum: top.sum + nums[top.last + 1],
                last: top.last + 1,
            });
            heap.push(Node {
                sum: top.sum - nums[top.last] + nums[top.last + 1],
                last: top.last + 1,
            });
        }

        total - result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2386() {
        assert_eq!(2, Solution::k_sum(vec![2, 4, -2], 5));
    }
}
