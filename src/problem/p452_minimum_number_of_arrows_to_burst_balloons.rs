/**
 * [452] Minimum Number of Arrows to Burst Balloons
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        if points.is_empty() {
            return 0;
        }

        let mut points : Vec<(i32, i32)> = points.iter().map(|p|{(p[0], p[1])}).collect();

        points.sort_unstable_by(|a, b| {
            a.1.cmp(&b.1)
        });

        let mut result = 0;
        let mut pos = points[0].1;

        for point in points {
            if point.0 > pos {
                pos = point.1;
                result += 1;
            }
        }

        result + 1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_452() {
    }
}
