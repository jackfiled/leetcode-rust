/**
 * [149] Max Points on a Line
 */
pub struct Solution {}


// submission codes start here
use std::collections::HashMap;

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Slope {
    x: i32,
    y: i32,
}

impl Slope {
    fn new(point1: &Vec<i32>, point2: &Vec<i32>) -> Self {
        let mut x = point1[0] - point2[0];
        let mut y = point1[1] - point2[1];

        if x == 0 {
            y = 1;
        } else if y == 0 {
            x = 1;
        } else {
            if y < 0 {
                x = -x;
                y = -y;
            }

            let gcd = Self::gcd(x.abs(), y.abs());
            x /= gcd;
            y /= gcd;
        }

        Self {
            x,
            y,
        }
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if b > 0 {
            Self::gcd(b, a % b)
        } else {
            a
        }
    }
}

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        if n <= 2 {
            return n as i32;
        }

        let mut result = 0;
        for i in 0..n {
            if result > n - i || result > n / 2 {
                break;
            }
            
            let mut map = HashMap::new();
            for j in i+1..n {
                let slope = Slope::new(&points[i], &points[j]);
                
                let entry = map.entry(slope).or_insert(0);
                *entry += 1;
            }
            
            let mut max = 0;
            for i in map.values() {
                max = max.max(*i);
            }
            
            result = result.max(max + 1);
        }

        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_149() {}
}
