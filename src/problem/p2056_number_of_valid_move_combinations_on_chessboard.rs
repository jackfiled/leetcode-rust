/**
 * [2056] Number of Valid Move Combinations On Chessboard
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, 0),
    (1, 0),
    (0, 1),
    (0, -1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

const SIZE: i32 = 8;

#[derive(Debug, Clone)]
struct Movement {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    step: i32,
}

impl Movement {
    fn new(x: i32, y: i32, dx: i32, dy: i32, step: i32) -> Self {
        Self { x, y, dx, dy, step }
    }

    fn generate_movements(x0: i32, y0: i32, directions: &[(i32, i32)]) -> Vec<Self> {
        let mut result = vec![Movement::new(x0, y0, 0, 0, 0)];

        for &(dx, dy) in directions {
            let mut x = x0 + dx;
            let mut y = y0 + dy;
            let mut step = 1;

            while x > 0 && x <= SIZE && y > 0 && y <= SIZE {
                result.push(Movement::new(x0, y0, dx, dy, step));
                x += dx;
                y += dy;
                step += 1;
            }
        }

        result
    }

    fn is_valid(&self, other: &Self) -> bool {
        let (mut x1, mut y1) = (self.x, self.y);
        let (mut x2, mut y2) = (other.x, other.y);

        for i in 0..self.step.max(other.step) {
            if i < self.step {
                x1 += self.dx;
                y1 += self.dy;
            }

            if i < other.step {
                x2 += other.dx;
                y2 += other.dy;
            }

            if x1 == x2 && y1 == y2 {
                return false;
            }
        }

        true
    }
}

impl Solution {
    pub fn count_combinations(pieces: Vec<String>, positions: Vec<Vec<i32>>) -> i32 {
        let n = pieces.len();
        let pieces_directions = Self::generate_pieces_directions();
        let all_movements: Vec<Vec<Movement>> = pieces
            .iter()
            .zip(positions.iter())
            .map(|(piece, position)| {
                Movement::generate_movements(
                    position[0],
                    position[1],
                    pieces_directions.get(piece.as_str()).unwrap(),
                )
            })
            .collect();

        let mut result = 0;
        let mut path = Vec::with_capacity(3);

        Self::dfs(&all_movements, &mut result, &mut path, 0, n);

        result
    }

    fn dfs(
        all_movements: &Vec<Vec<Movement>>,
        result: &mut i32,
        path: &mut Vec<Movement>,
        i: usize,
        n: usize,
    ) {
        if i == n {
            *result += 1;
            return;
        }

        for movement in all_movements[i].iter() {
            if (0..i).all(|i| movement.is_valid(&path[i])) {
                path.push(movement.clone());
                Self::dfs(all_movements, result, path, i + 1, n);
                path.pop();
            }
        }
    }

    fn generate_pieces_directions() -> HashMap<&'static str, &'static [(i32, i32)]> {
        let mut result = HashMap::with_capacity(3);

        result.insert("rook", &DIRECTIONS[..4]);
        result.insert("bishop", &DIRECTIONS[4..]);
        result.insert("queen", DIRECTIONS.as_slice());

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2056() {
        assert_eq!(
            15,
            Solution::count_combinations(vec_string!("rook"), vec![vec![1, 1]])
        );
        assert_eq!(
            22,
            Solution::count_combinations(vec_string!("queen"), vec![vec![1, 1]])
        );
        assert_eq!(
            12,
            Solution::count_combinations(vec_string!("bishop"), vec![vec![4, 3]])
        );
    }
}
