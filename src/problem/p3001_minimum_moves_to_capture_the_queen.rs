/**
 * [3001] Minimum Moves to Capture The Queen
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn min_moves_to_capture_the_queen(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32 {
        let rook_min_path = if a == e || b == f {
            if Self::check_on_path(c, d, a, b, e, f) {
                2
            } else {
                1
            }
        } else {
            2
        };

        let bishop_min_path = if c.abs_diff(e) == d.abs_diff(f) {
            if Self::check_on_path(a, b, c, d, e, f) {
                2
            } else {
                1
            }
        } else {
            2
        };

        rook_min_path.min(bishop_min_path)
    }

    fn check_on_path(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> bool {
        let delta = c.abs_diff(e).max(d.abs_diff(f)) as i32;
        let (dx, dy) = ((e - c) / delta, (f - d) / delta);

        if (1..=delta)
            .map(|i| (c + (i * dx), d + (i * dy)))
            .any(|(x, y)| x == a && y == b)
        {
            true
        } else {
            false
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3001() {
        assert_eq!(
            1,
            Solution::min_moves_to_capture_the_queen(7, 4, 1, 4, 7, 8)
        );
        assert_eq!(
            2,
            Solution::min_moves_to_capture_the_queen(4, 3, 3, 4, 5, 2)
        );
        assert_eq!(
            2,
            Solution::min_moves_to_capture_the_queen(1, 1, 8, 8, 2, 3)
        );
        assert_eq!(
            1,
            Solution::min_moves_to_capture_the_queen(5, 3, 3, 4, 5, 2)
        );
    }
}
