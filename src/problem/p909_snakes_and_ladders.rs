/**
 * [909] Snakes and Ladders
 */
pub struct Solution {}

// submission codes start here
use std::collections::VecDeque;

impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len() * board.len();
        let mut map = Vec::with_capacity(n);
        map.push(0);

        let mut reverse = false;
        let mut index = 1;

        for array in board.iter().rev() {
            let array: Box<dyn Iterator<Item = &i32>> = if reverse {
                Box::new(array.iter().rev())
            } else {
                Box::new(array.iter())
            };

            for &i in array {
                if i == -1 {
                    map.push(index);
                } else {
                    map.push(i as usize);
                }

                index += 1;
            }

            reverse = !reverse;
        }

        let mut visited = vec![false; n + 1];
        let mut queue = VecDeque::new();
        queue.push_back(1);

        let mut count = 0;
        while !queue.is_empty() {
            let length = queue.len();

            for _ in 0..length {
                let node = queue.pop_front().unwrap();

                if node == n {
                    return count;
                }

                for next in (node + 1)..=(node + 6) {
                    if next > n {
                        continue;
                    }

                    if !visited[next] {
                        visited[next] = true;
                        queue.push_back(map[next]);
                    }
                }
            }

            count += 1;
        }

        -1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_909_1() {
        assert_eq!(
            Solution::snakes_and_ladders(vec![
                vec![-1, -1, -1, -1, -1, -1],
                vec![-1, -1, -1, -1, -1, -1],
                vec![-1, -1, -1, -1, -1, -1],
                vec![-1, 35, -1, -1, 13, -1],
                vec![-1, -1, -1, -1, -1, -1],
                vec![-1, 15, -1, -1, -1, -1]
            ]),
            4
        );
    }

    #[test]
    fn test_909_2() {
        assert_eq!(
            Solution::snakes_and_ladders(vec![vec![-1, -1, -1], vec![-1, 9, 8], vec![-1, 8, 9]]),
            1
        );
    }
}
