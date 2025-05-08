/**
 * [3342] Find Minimum Time to Reach Last Room II
 */
pub struct Solution {}

// submission codes start here
use std::cmp::Ordering;
use std::collections::BinaryHeap;

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord)]
struct State {
    x: usize,
    y: usize,
    distance: i32,
    index: i32,
}

impl State {
    fn new(x: usize, y: usize, distance: i32, index: i32) -> Self {
        Self {
            x,
            y,
            distance,
            index,
        }
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // BinaryHeap是大顶堆
        Some(other.distance.cmp(&self.distance))
    }
}

impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        let n = move_time.len();
        let m = move_time[0].len();

        let mut distances = vec![vec![i32::MAX; m]; n];
        let mut visited = vec![vec![false; m]; n];
        let mut heap = BinaryHeap::new();

        distances[0][0] = 0;
        heap.push(State::new(0, 0, 0, 0));

        while let Some(head) = heap.pop() {
            if visited[head.x][head.y] {
                continue;
            }
            visited[head.x][head.y] = true;

            for &(dx, dy) in DIRECTIONS.iter() {
                let next = head
                    .x
                    .checked_add_signed(dx)
                    .and_then(|x| head.y.checked_add_signed(dy).and_then(|y| Some((x, y))))
                    .filter(|(x, y)| x < &n && y < &m);

                if let Some((x, y)) = next {
                    let distance = distances[head.x][head.y].max(move_time[x][y])
                        + if head.index % 2 == 0 { 1 } else { 2 };

                    if distances[x][y] > distance {
                        distances[x][y] = distance;
                        heap.push(State::new(x, y, distance, head.index + 1));
                    }
                }
            }
        }

        distances[n - 1][m - 1]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3342() {
        assert_eq!(7, Solution::min_time_to_reach(vec![vec![0, 4], vec![4, 4]]));
        assert_eq!(
            6,
            Solution::min_time_to_reach(vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0]])
        );
        assert_eq!(4, Solution::min_time_to_reach(vec![vec![0, 1], vec![1, 2]]));
    }
}
