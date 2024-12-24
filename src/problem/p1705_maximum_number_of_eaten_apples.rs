/**
 * [1705] Maximum Number of Eaten Apples
 */
pub struct Solution {}

// submission codes start here
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn eaten_apples(apples: Vec<i32>, days: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut heap: BinaryHeap<(Reverse<i32>, i32)> = BinaryHeap::new();
        let n = apples.len() as i32;

        for (day, (count, rot)) in apples
            .into_iter()
            .zip(days.into_iter())
            .enumerate()
            .map(|(i, v)| (i as i32 + 1, v))
        {
            while let Some(head) = heap.peek() {
                if head.0 .0 <= day {
                    heap.pop();
                } else {
                    break;
                }
            }

            let rotten_day = day + rot;
            if count > 0 {
                heap.push((Reverse(rotten_day), count));
            }

            if let Some((rotted_day, count)) = heap.pop() {
                if count > 1 {
                    heap.push((rotted_day, count - 1));
                }
                result += 1;
            }
        }

        let mut day = n + 1;

        while let Some((rotted_day, count)) = heap.pop() {
            if rotted_day.0 <= day {
                continue;
            }

            let get_count = count.min(rotted_day.0 - day);
            result += get_count;
            day += get_count;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1705() {
        assert_eq!(
            7,
            Solution::eaten_apples(vec![1, 2, 3, 5, 2], vec![3, 2, 1, 4, 2])
        );
        assert_eq!(
            5,
            Solution::eaten_apples(vec![3, 0, 0, 0, 0, 2], vec![3, 0, 0, 0, 0, 2])
        );
    }
}
