/**
 * [2617] Minimum Number of Visited Cells in a Grid
 */
pub struct Solution {}

// submission codes start here
use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn minimum_visited_cells(grid: Vec<Vec<i32>>) -> i32 {
        let height = grid.len();
        let width = grid[0].len();

        let mut column_heaps: Vec<BinaryHeap<(Reverse<i32>, usize)>> =
            vec![BinaryHeap::new(); width];
        let mut row_heap: BinaryHeap<(Reverse<i32>, usize)> = BinaryHeap::new();

        let mut dis = 0;
        for (i, row) in grid.iter().enumerate() {
            row_heap.clear();

            for (j, &node) in row.iter().enumerate() {
                while !row_heap.is_empty() && row_heap.peek().unwrap().1 < j {
                    row_heap.pop();
                }

                let column_heap = &mut column_heaps[j];
                while !column_heap.is_empty() && column_heap.peek().unwrap().1 < i {
                    column_heap.pop();
                }

                dis = if i > 0 || j > 0 { i32::MAX } else { 1 };

                if let Some((d, _)) = row_heap.peek() {
                    dis = d.0 + 1;
                }

                if let Some((d, _)) = column_heap.peek() {
                    dis = dis.min(d.0 + 1);
                }

                if node > 0 && dis < i32::MAX {
                    let node = node as usize;
                    row_heap.push((Reverse(dis), node + j));
                    column_heap.push((Reverse(dis), node + i));
                }
            }
        }

        if dis < i32::MAX {
            dis
        } else {
            -1
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2617() {}
}
