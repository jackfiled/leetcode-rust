/**
 * [1298] Maximum Candies You Can Get from Boxes
 */
pub struct Solution {}

// submission codes start here
use std::collections::{HashSet, VecDeque};
use std::iter::FromIterator;

impl Solution {
    pub fn max_candies(
        status: Vec<i32>,
        candies: Vec<i32>,
        keys: Vec<Vec<i32>>,
        contained_boxes: Vec<Vec<i32>>,
        initial_boxes: Vec<i32>,
    ) -> i32 {
        let mut result = 0;
        let mut boxes_in_hand: HashSet<usize> =
            HashSet::from_iter(initial_boxes.into_iter().map(|x| x as usize));
        let mut can_open_boxes: HashSet<usize> =
            HashSet::from_iter(status.iter().enumerate().filter_map(|(i, v)| {
                if *v == 1 {
                    Some(i)
                } else {
                    None
                }
            }));
        let mut opened_boxes = HashSet::new();
        let mut queue = VecDeque::new();

        for &b in boxes_in_hand.iter().filter(|x| status[**x] == 1) {
            queue.push_back(b);
            opened_boxes.insert(b);
            result += candies[b];
        }

        while let Some(b) = queue.pop_front() {
            for &key in keys[b].iter() {
                let key = key as usize;
                can_open_boxes.insert(key);

                if !opened_boxes.contains(&key) && boxes_in_hand.contains(&key) {
                    queue.push_back(key);
                    opened_boxes.insert(key);
                    result += candies[key];
                }
            }

            for &new_box in contained_boxes[b].iter() {
                let new_box = new_box as usize;
                boxes_in_hand.insert(new_box);

                if !opened_boxes.contains(&new_box) && can_open_boxes.contains(&new_box) {
                    queue.push_back(new_box);
                    opened_boxes.insert(new_box);
                    result += candies[new_box];
                }
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
    fn test_1298() {
        assert_eq!(
            16,
            Solution::max_candies(
                vec![1, 0, 1, 0],
                vec![7, 5, 4, 1000],
                vec![vec![], vec![], vec![1], vec![]],
                vec![vec![1, 2], vec![3], vec![], vec![]],
                vec![0]
            )
        );
        assert_eq!(
            6,
            Solution::max_candies(
                vec![1, 0, 0, 0, 0, 0],
                vec![1, 1, 1, 1, 1, 1],
                vec![vec![1, 2, 3, 4, 5], vec![], vec![], vec![], vec![], vec![]],
                vec![vec![1, 2, 3, 4, 5], vec![], vec![], vec![], vec![], vec![]],
                vec![0]
            )
        );
    }
}
