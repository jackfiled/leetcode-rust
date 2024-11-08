/**
 * [3235] Check if the Rectangle Corner Is Reachable
 */
pub struct Solution {}

// submission codes start here

struct Circle {
    x: i64,
    y: i64,
    radius: i64,
}

impl Circle {
    fn new(array: Vec<i32>) -> Self {
        Circle {
            x: array[0] as i64,
            y: array[1] as i64,
            radius: array[2] as i64,
        }
    }

    fn intersect_in_rectangle(a: &Self, b: &Self, x: i64, y: i64) -> bool {
        (a.x - b.x).pow(2) + (a.y - b.y).pow(2) <= (a.radius + b.radius).pow(2)
            && a.x * b.radius + b.x * a.radius <= (a.radius + b.radius) * x
            && a.y * b.radius + b.y * a.radius <= (a.radius + b.radius) * y
    }

    fn contains_point(&self, x: i64, y: i64) -> bool {
        (x - self.x).pow(2) + (y - self.y).pow(2) <= self.radius.pow(2)
    }

    fn intersect_with_top_left(&self, x: i64, y: i64) -> bool {
        (self.x.abs() <= self.radius && 0i64 <= self.y && self.y <= y)
            || ((self.y - y).abs() <= self.radius && 0i64 <= self.x && self.x <= x)
            || self.contains_point(x, y)
    }

    fn intersect_with_bottom_right(&self, x: i64, y: i64) -> bool {
        ((x - self.x).abs() <= self.radius && 0i64 <= self.y && self.y <= y)
            || (self.y.abs() <= self.radius && 0i64 <= self.x && self.x <= x)
            || self.contains_point(x, y)
    }
}

impl Solution {
    pub fn can_reach_corner(x_corner: i32, y_corner: i32, circles: Vec<Vec<i32>>) -> bool {
        let n = circles.len();
        let (x_corner, y_corner) = (x_corner as i64, y_corner as i64);
        let circles: Vec<Circle> = circles.into_iter().map(|v| Circle::new(v)).collect();

        let mut visited = vec![false; n];

        for (i, c) in circles.iter().enumerate() {
            if c.contains_point(0i64, 0i64) || c.contains_point(x_corner, y_corner) {
                return false;
            }

            if !visited[i] && c.intersect_with_top_left(x_corner, y_corner) {
                if Self::dfs(i, &circles, &mut visited, x_corner, y_corner) {
                    return false;
                }
            }
        }

        true
    }

    fn dfs(
        i: usize,
        circles: &Vec<Circle>,
        visited: &mut Vec<bool>,
        x_corner: i64,
        y_corner: i64,
    ) -> bool {
        if circles[i].intersect_with_bottom_right(x_corner, y_corner) {
            return true;
        }

        visited[i] = true;
        for (j, c) in circles.iter().enumerate() {
            if visited[j] {
                continue;
            }

            if Circle::intersect_in_rectangle(&circles[i], c, x_corner, y_corner) {
                if Self::dfs(j, circles, visited, x_corner, y_corner) {
                    return true;
                }
            }
        }

        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3235() {
        assert!(Solution::can_reach_corner(
            5_0000_0000,
            5_0000_0000,
            vec![
                vec![4_9998_0000, 6_9999_9999, 2_0000_0000],
                vec![5_0002_0000, 3_0000_0001, 2_0000_0000],
            ]
        ));
        assert!(!Solution::can_reach_corner(
            15,
            15,
            vec![vec![2, 20, 13], vec![20, 2, 13]]
        ));
        assert!(Solution::can_reach_corner(
            3,
            3,
            vec![vec![2, 1000, 997], vec![1000, 2, 997]]
        ));
        assert!(!Solution::can_reach_corner(5, 7, vec![vec![2, 2, 7]]));
        assert!(Solution::can_reach_corner(3, 4, vec![vec![2, 1, 1]]));
        assert!(!Solution::can_reach_corner(3, 3, vec![vec![1, 1, 2]]));
        assert!(!Solution::can_reach_corner(
            3,
            3,
            vec![vec![2, 1, 1], vec![1, 2, 1]]
        ));
        assert!(Solution::can_reach_corner(4, 4, vec![vec![5, 5, 1]]));
    }
}
